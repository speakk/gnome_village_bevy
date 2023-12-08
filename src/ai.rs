//! This example describes how to create an action that takes multiple steps.
//!
//! It is similar to the thirst example, but instead of just magically quenching a thirst,
//! the actor must be near a water source in order to drink.
//!
//! Note that it does not matter if the actor is already near a water source:
//! the MoveToWaterSource action will simply terminate immediately.

use crate::components::{Blueprint, BuildingProcess, Settler};
use crate::map::LayerBuilding;
use crate::systems::blueprint::BlueprintFinished;
use bevy::prelude::*;
use bevy::utils::tracing::{debug, trace};
use bevy_ecs_tilemap::map::TilemapId;
use bevy_rapier2d::prelude::*;
use big_brain::prelude::*;

#[derive(Component, Debug)]
pub struct BuildingNeed {
    /// How much thirst the entity currently has.
    pub building_need: f32,
}

impl BuildingNeed {
    pub fn new(building_need: f32) -> Self {
        Self { building_need }
    }
}

/// A simple system that just pushes the thirst value up over time.
/// Just a plain old Bevy system, big-brain is not involved yet.
pub fn building_need_system(mut building_needs: Query<&mut BuildingNeed>) {
    for mut building_need in &mut building_needs {
        building_need.building_need = 0.5;
    }
}

/// An action where the actor moves to the closest water source
#[derive(Clone, Component, Debug, ActionBuilder)]
pub struct MoveToBlueprint {
    // The movement speed of the actor.
    pub speed: f32,
}

/// Closest distance to a water source to be able to drink from it.
const MAX_DISTANCE: f32 = 0.1;

pub fn move_to_blueprint_action_system(
    time: Res<Time>,
    // Find all water sources
    blueprints: Query<(&Transform, Entity), With<Blueprint>>,
    // We use Without to make disjoint queries.
    mut positions: Query<
        (&mut Transform, &mut KinematicCharacterController),
        (With<Settler>, Without<Blueprint>),
    >,
    // A query on all current MoveToWaterSource actions.
    mut action_query: Query<(&Actor, &mut ActionState, &MoveToBlueprint, &ActionSpan)>,
) {
    println!("Blueprint length: {}", blueprints.iter().count());
    // Loop through all actions, just like you'd loop over all entities in any other query.
    for (actor, mut action_state, move_to, span) in &mut action_query {
        let _guard = span.span().enter();

        // Different behavior depending on action state.
        match *action_state {
            // Action was just requested; it hasn't been seen before.
            ActionState::Requested => {
                //println!("Let's go find some water!");
                // We don't really need any initialization code here, since the queries are cheap enough.
                *action_state = ActionState::Executing;
            }
            ActionState::Executing => {
                // Look up the actor's position.
                let (actor_position, mut kinematic_controller) =
                    positions.get_mut(actor.0).expect("actor has no position");

                //println!("Actor position: {:?}", actor_position.translation);

                // Look up the water source closest to them.
                if let Some((closest_blueprint_transform, _)) =
                    find_closest_blueprint(&blueprints, &actor_position)
                {
                    let final_blueprint_transform = closest_blueprint_transform;
                    // Find how far we are from it.
                    let delta = final_blueprint_transform.translation - actor_position.translation;

                    let distance = delta.length();

                    //println!("Distance: {}", distance);

                    if distance > MAX_DISTANCE {
                        // We're still too far, take a step toward it!

                        trace!("Stepping closer.");

                        // How far can we travel during this frame?
                        let step_size = time.delta_seconds() * move_to.speed;
                        // Travel towards the water-source position, but make sure to not overstep it.
                        let step = delta.normalize() * step_size.min(distance);

                        // Move the actor.
                        //actor_position.translation += step;
                        kinematic_controller.translation = Some(step.truncate());
                    } else {
                        // We're within the required distance! We can declare success.

                        debug!("We got there!");

                        // The action will be cleaned up automatically.
                        *action_state = ActionState::Success;
                    }
                } else {
                    *action_state = ActionState::Failure;
                }
            }
            ActionState::Cancelled => {
                // Always treat cancellations, or we might keep doing this forever!
                // You don't need to terminate immediately, by the way, this is only a flag that
                // the cancellation has been requested. If the actor is balancing on a tightrope,
                // for instance, you may let them walk off before ending the action.
                *action_state = ActionState::Failure;
            }
            _ => {}
        }
    }
}

/// A utility function that finds the closest water source to the actor.
fn find_closest_blueprint(
    blueprints: &Query<(&Transform, Entity), With<Blueprint>>,
    actor_position: &Transform,
) -> Option<(Transform, Entity)> {
    let Some((transform, entity)) = blueprints.iter().min_by(|(a, _), (b, _)| {
        let da = (a.translation - actor_position.translation).length_squared();
        let db = (b.translation - actor_position.translation).length_squared();
        da.partial_cmp(&db).unwrap()
    }) else {
        return None;
    };

    return Some((transform.clone(), entity.clone()));
}

/// A simple action: the actor's thirst shall decrease, but only if they are near a water source.
#[derive(Clone, Component, Debug, ActionBuilder)]
pub struct Build {
    pub per_second: f32,
}

pub fn build_action_system(
    time: Res<Time>,
    mut building_needs: Query<&Transform, (With<Settler>, With<BuildingNeed>)>,
    blueprint_query: Query<(&Transform, Entity), With<Blueprint>>,
    mut building_processes: Query<&mut BuildingProcess, With<Blueprint>>,
    mut query: Query<(&Actor, &mut ActionState, &Build, &ActionSpan)>,
    mut blueprint_finished_event_writer: EventWriter<BlueprintFinished>,
    tilemap_query: Query<
        (&Transform, &TilemapId),
        (Without<Blueprint>, Without<Settler>, With<LayerBuilding>),
    >,
) {
    // Loop through all actions, just like you'd loop over all entities in any other query.
    for (Actor(actor), mut state, build, span) in &mut query {
        let _guard = span.span().enter();

        // Look up the actor's position and thirst from the Actor component in the action entity.
        let actor_position = building_needs
            .get_mut(*actor)
            .expect("actor has no building need");

        match *state {
            ActionState::Requested => {
                // We'll start drinking as soon as we're requested to do so.
                debug!("Building");
                *state = ActionState::Executing;
            }
            ActionState::Executing => {
                // Look up the closest water source.
                // Note that there is no explicit passing of a selected water source from the GoToWaterSource action,
                // so we look it up again. Note that this decouples the actions from each other,
                // so if the actor is already close to a water source, the GoToWaterSource action
                // will not be necessary (though it will not harm either).
                //
                // Essentially, being close to a water source would be a precondition for the Drink action.
                // How this precondition was fulfilled is not this code's concern.
                if let Some((closest_blueprint_transform, closest_blueprint_entity)) =
                    find_closest_blueprint(&blueprint_query, actor_position)
                {
                    // Find how far we are from it.
                    let (map_transform, _) = tilemap_query.iter().last().unwrap();
                    let final_blueprint_transform = *map_transform * closest_blueprint_transform;
                    let distance = (final_blueprint_transform.translation
                        - actor_position.translation)
                        .length();

                    // Are we close enough?
                    if distance < MAX_DISTANCE {
                        println!("Building!");

                        // Start reducing the thirst. Alternatively, you could send out some kind of
                        // DrinkFromSource event that indirectly decreases thirst.
                        //thirst.thirst -= drink.per_second * time.delta_seconds();
                        let mut building_process = building_processes
                            .get_mut(closest_blueprint_entity)
                            .expect("Blueprint has no BuildingProcess");

                        building_process.process += build.per_second * time.delta_seconds();
                        if building_process.process >= 1.0 {
                            building_process.process = 1.0;
                            blueprint_finished_event_writer
                                .send(BlueprintFinished(closest_blueprint_entity));
                            *state = ActionState::Success;
                        }
                    } else {
                        // The actor was told to drink, but they can't drink when they're so far away!
                        // The action doesn't know how to deal with this case, it's the overarching system's
                        // to fulfill the precondition.
                        debug!("We're too far away!");
                        *state = ActionState::Failure;
                    }
                } else {
                    *state = ActionState::Failure;
                }
            }
            // All Actions should make sure to handle cancellations!
            // Drinking is not a complicated action, so we can just interrupt it immediately.
            ActionState::Cancelled => {
                *state = ActionState::Failure;
            }
            _ => {}
        }
    }
}

// Scorers are the same as in the thirst example.
#[derive(Clone, Component, Debug, ScorerBuilder)]
pub struct BuildingNeedy;

pub fn building_needy_scorer_system(
    building_needs: Query<&BuildingNeed>,
    mut query: Query<(&Actor, &mut Score), With<BuildingNeedy>>,
) {
    for (Actor(actor), mut score) in &mut query {
        if let Ok(building_need) = building_needs.get(*actor) {
            score.set(building_need.building_need);
        }
    }
}

// fn main() {
//     // Once all that's done, we just add our systems and off we go!
//     App::new()
//         .add_plugins(DefaultPlugins.set(LogPlugin {
//             // Use `RUST_LOG=big_brain=trace,thirst=trace cargo run --example thirst --features=trace` to see extra tracing output.
//             filter: "big_brain=debug,sequence=debug".to_string(),
//             ..default()
//         }))
//         .add_plugins(BigBrainPlugin::new(PreUpdate))
//         .add_systems(Startup, init_entities)
//         .add_systems(Update, thirst_system)
//         .add_systems(
//             PreUpdate,
//             (drink_action_system, move_to_water_source_action_system).in_set(BigBrainSet::Actions),
//         )
//         .add_systems(First, thirsty_scorer_system)
//         .run();
// }
//
