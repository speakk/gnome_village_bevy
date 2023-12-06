mod camera;
mod components;
mod map;
mod placement;

use crate::camera::setup_camera;
use crate::components::Settler;
use bevy::prelude::*;
use camera::camera_movement;
use map::tilemap_setup;
use placement::{placement, update_cursor_pos, CursorPos};
use rand::Rng;

use bevy_ecs_tilemap::prelude::*;

fn main() {
    App::new()
        .init_resource::<CursorPos>()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(TilemapPlugin)
        .add_systems(Startup, (setup_camera, misc_setup, tilemap_setup))
        .add_systems(Update, camera_movement)
        .add_systems(Update, update_cursor_pos)
        .add_systems(Update, placement)
        .add_systems(FixedUpdate, walkie)
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
    for _n in 1..2 {
        commands.spawn((
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle.clone(),
                sprite: TextureAtlasSprite::new(animation_indices.first),
                transform: Transform::from_xyz(
                    rng.gen_range(-300.0..300.0),
                    rng.gen_range(-300.0..300.0),
                    0.,
                ),
                ..default()
            },
            animation_indices.clone(),
            AnimationTimer(Timer::from_seconds(0.15, TimerMode::Repeating)),
            Settler,
        ));
    }
}

fn walkie(time: Res<Time>, mut sprite_position: Query<&mut Transform, With<Settler>>) {
    let mut rng = rand::thread_rng();
    for mut transform in &mut sprite_position {
        transform.translation.x += rng.gen_range(-20.0..20.0) * time.delta_seconds();
        transform.translation.y += rng.gen_range(-20.0..20.0) * time.delta_seconds();
    }
}

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
