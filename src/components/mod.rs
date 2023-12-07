use bevy::prelude::*;

#[derive(Component)]
pub struct Settler;

#[derive(Component)]
pub struct Blueprint;

#[derive(Clone, Component, Debug)]
pub struct BuildingProcess {
    pub process: f32,
}
