use bevy::prelude::*;

#[derive(Component)]
pub struct Settler;

#[derive(Component)]
pub struct Blueprint;

#[derive(Clone, Component, Debug)]
pub struct BuildingProcess {
    pub process: f32,
}

#[derive(Clone, Component, Debug)]
pub struct JobTarget {
    pub current_workers: usize,
    pub max_workers: usize,
}

impl Default for JobTarget {
    fn default() -> Self {
        JobTarget {
            max_workers: 3,
            current_workers: 0,
        }
    }
}
