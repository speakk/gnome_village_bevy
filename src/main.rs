mod camera;
mod components;
mod map;
mod placement;
mod plugins;
mod systems;

use crate::camera::setup_camera;
use crate::components::Settler;
use crate::plugins::tasks::TasksPlugin;
use bevy::prelude::*;
use camera::camera_movement;
use map::tilemap_setup;
use placement::{placement, update_cursor_pos, CursorPos};
use rand::Rng;

use bevy_ecs_tilemap::prelude::*;
use bevy_rapier2d::prelude::*;
use big_brain::prelude::*;
use systems::blueprint::blueprint;
use systems::blueprint::BlueprintFinished;

fn main() {
    App::new()
        .init_resource::<CursorPos>()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(TilemapPlugin)
        .add_plugins(TasksPlugin)
        .add_plugins(BigBrainPlugin::new(PreUpdate))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        //.add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, (setup_camera, misc_setup, tilemap_setup))
        .add_systems(Update, camera_movement)
        .add_systems(Update, update_cursor_pos)
        .add_systems(Update, placement)
        .add_systems(Update, blueprint)
        .add_event::<BlueprintFinished>()
        //.add_systems(FixedUpdate, walkie)
        .add_systems(FixedUpdate, animate_sprite)
        .run();
}

fn misc_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let mut rng = rand::thread_rng();
    let texture_handle = asset_server.load("settler.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(8.0, 8.0), 4, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let animation_indices = AnimationIndices { first: 0, last: 3 };
    for _n in 1..4 {
        commands.spawn((
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle.clone(),
                sprite: TextureAtlasSprite::new(animation_indices.first),
                transform: Transform::from_xyz(
                    rng.gen_range(-100.0..100.0),
                    rng.gen_range(-100.0..100.0),
                    0.2,
                ),
                ..default()
            },
            animation_indices.clone(),
            AnimationTimer(Timer::from_seconds(0.15, TimerMode::Repeating)),
            Settler,
            Collider::ball(0.1),
            KinematicCharacterController::default(),
            RigidBody::KinematicPositionBased,
        ));
    }
}

// fn walkie(time: Res<Time>, mut sprite_position: Query<&mut Transform, With<Settler>>) {
//     let mut rng = rand::thread_rng();
//     for mut transform in &mut sprite_position {
//         transform.translation.x += rng.gen_range(-2.0..2.0) * time.delta_seconds();
//         transform.translation.y += rng.gen_range(-2.0..2.0) * time.delta_seconds();
//     }
// }

#[derive(Component, Clone)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(
        &AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = if sprite.index == indices.last {
                indices.first
            } else {
                sprite.index + 1
            };
        }
    }
}

// #[derive(Resource)]
// struct TileMapId {
//     id: TilemapId
//     bonus: u32,
// }
