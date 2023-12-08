use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::{
    components::{Blueprint, BuildingProcess},
    map::LayerBuilding,
};

#[derive(Resource)]
pub struct CursorPos(Vec2);
impl Default for CursorPos {
    fn default() -> Self {
        // Initialize the cursor pos at some far away place. It will get updated
        // correctly when the cursor moves.
        Self(Vec2::new(-1000.0, -1000.0))
    }
}

// We need to keep the cursor position updated based on any `CursorMoved` events.
pub fn update_cursor_pos(
    camera_q: Query<(&GlobalTransform, &Camera)>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut cursor_pos: ResMut<CursorPos>,
) {
    for cursor_moved in cursor_moved_events.read() {
        // To get the mouse's world position, we have to transform its window position by
        // any transforms on the camera. This is done by projecting the cursor position into
        // camera space (world space).
        for (cam_t, cam) in camera_q.iter() {
            if let Some(pos) = cam.viewport_to_world_2d(cam_t, cursor_moved.position) {
                *cursor_pos = CursorPos(pos);
            }
        }
    }
}

// This is where we check which tile the cursor is hovered over.
pub fn placement(
    mut commands: Commands,
    cursor_pos: Res<CursorPos>,
    mut tilemap_q: Query<
        (
            &TilemapSize,
            &TilemapGridSize,
            &TilemapType,
            &TilemapId,
            &mut TileStorage,
            &Transform,
        ),
        With<LayerBuilding>,
    >,
    mouse_button: Res<Input<MouseButton>>,
) {
    if mouse_button.pressed(MouseButton::Left) {
        for (map_size, grid_size, map_type, tilemap_id, mut tile_storage, map_transform) in
            tilemap_q.iter_mut()
        {
            // Grab the cursor position from the `Res<CursorPos>`
            let cursor_pos: Vec2 = cursor_pos.0;
            // We need to make sure that the cursor's world position is correct relative to the map
            // due to any map transformation.
            let cursor_in_map_pos: Vec2 = {
                // Extend the cursor_pos vec3 by 0.0 and 1.0
                let cursor_pos = Vec4::from((cursor_pos, 0.0, 1.0));
                let cursor_in_map_pos = map_transform.compute_matrix().inverse() * cursor_pos;
                cursor_in_map_pos.xy()
            };
            // Once we have a world position we can transform it into a possible tile position.
            if let Some(tile_pos) =
                TilePos::from_world_pos(&cursor_in_map_pos, map_size, grid_size, map_type)
            {
                let world_pos = tile_pos.center_in_world(grid_size, map_type);
                // Highlight the relevant tile's label
                if let Some(old_tile_entity) = tile_storage.get(&tile_pos) {
                    commands.entity(old_tile_entity).despawn_recursive();
                }
                let tile_entity = commands
                    .spawn(TileBundle {
                        position: tile_pos,
                        texture_index: TileTextureIndex(1),
                        color: TileColor(Color::Rgba {
                            red: 0.5,
                            green: 0.5,
                            blue: 1.0,
                            alpha: 0.6,
                        }),
                        tilemap_id: *tilemap_id,
                        ..Default::default()
                    })
                    .insert(Blueprint)
                    .insert(BuildingProcess { process: 0.0 })
                    .insert(Transform::from_xyz(world_pos.x, world_pos.y, 0.0))
                    .id();

                tile_storage.set(&tile_pos, tile_entity);

                //commands.entity(tile_entity).insert(HighlightedLabel);
            }
        }
    }
}
