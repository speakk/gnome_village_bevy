use bevy::prelude::*;

use crate::components::{Blueprint, Settler};

pub struct TasksPlugin;

enum ResourceType {
    Wood,
    Iron,
}

struct ResourceRequirement {
    resource_type: ResourceType,
    amount: u32,
}

enum TaskType {
    Build {
        target: Entity,
    },
    BringResources {
        resources: Vec<ResourceRequirement>,
        target: Entity,
    },
}

#[derive(Component)]
struct Task {
    task_type: TaskType,
    finished: bool,
}

impl Task {
    pub fn new(task_type: TaskType) -> Self {
        Self {
            task_type,
            finished: false,
        }
    }
}

#[derive(Component)]
struct AssignedTask {
    assigned_to: Entity,
}

#[derive(Component)]
struct SettlerTask {
    task_reference: Entity,
}

#[derive(Component)]
struct TaskSequence {
    sequence: Vec<Task>,
}

fn task_assignment(
    tasks: Query<Entity, (Without<AssignedTask>, With<Task>)>,
    settlers: Query<Entity, (With<Settler>, Without<SettlerTask>)>,
    mut commands: Commands,
) {
    for task_entity in tasks.iter() {
        // TODO: For now just pick first settler, but this is where we organize settlers by scoring
        // by proximity to task etc
        if let Some(settler_entity) = settlers.iter().next() {
            commands.entity(settler_entity).insert(SettlerTask {
                task_reference: task_entity,
            });
            commands.entity(task_entity).insert(AssignedTask {
                assigned_to: settler_entity,
            });
        }
    }
}

pub fn generate_blueprint_tasks(
    mut commands: Commands,
    blueprint_placed: Query<Entity, Added<Blueprint>>,
) {
    for blueprint in blueprint_placed.iter() {
        let task_sequence_items = vec![
            Task::new(TaskType::BringResources {
                resources: vec![],
                target: blueprint,
            }),
            Task::new(TaskType::Build { target: blueprint }),
        ];
        commands.spawn(TaskSequence {
            sequence: task_sequence_items,
        });
    }
}

impl Plugin for TasksPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, task_assignment);
        app.add_systems(Update, generate_blueprint_tasks);
    }
}
