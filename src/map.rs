use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

pub const TILE_SIZE_X: f32 = 8.0;
pub const TILE_SIZE_Y: f32 = 8.0;

pub const MAP_SIZE_X: u32 = 32;
pub const MAP_SIZE_Y: u32 = 32;

#[derive(Component)]
pub struct LayerGround;

#[derive(Component)]
pub struct LayerBuilding;

pub fn tilemap_setup(asset_server: Res<AssetServer>, mut commands: Commands) {
    let texture_handle = asset_server.load("tilesheet.png");

    let map_size = TilemapSize {
        x: MAP_SIZE_X,
        y: MAP_SIZE_Y,
    };

    let tile_size = TilemapTileSize {
        x: TILE_SIZE_X,
        y: TILE_SIZE_Y,
    };

    let grid_size = tile_size.into();
    let map_type = TilemapType::default();

    // Layer 1 (ground)
    let mut tile_storage = TileStorage::empty(map_size);
    let tilemap_entity = commands.spawn_empty().id();

    fill_tilemap(
        TileTextureIndex(0),
        map_size,
        TilemapId(tilemap_entity),
        &mut commands,
        &mut tile_storage,
    );

    // Set up tilemap
    let tilemap_bundle = TilemapBundle {
        texture: TilemapTexture::Single(texture_handle.clone()),
        grid_size,
        map_type,
        tile_size,
        size: map_size,
        storage: tile_storage,
        transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.1),
        ..Default::default()
    };

    // Spawn tilemap
    commands
        .entity(tilemap_entity)
        .insert(tilemap_bundle)
        .insert(LayerGround {})
        .insert(TilemapId(tilemap_entity));

    // Layer 2 (buildings)
    let tile_storage = TileStorage::empty(map_size);
    let tilemap_entity = commands.spawn_empty().id();

    // fill_tilemap(
    //     TileTextureIndex(0),
    //     map_size,
    //     TilemapId(tilemap_entity),
    //     &mut commands,
    //     &mut tile_storage,
    // );

    // Set up tilemap
    let tilemap_bundle = TilemapBundle {
        texture: TilemapTexture::Single(texture_handle.clone()),
        grid_size,
        map_type,
        tile_size,
        size: map_size,
        storage: tile_storage,
        transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.2),
        ..Default::default()
    };

    // Spawn tilemap
    commands
        .entity(tilemap_entity)
        .insert(tilemap_bundle)
        .insert(LayerBuilding {})
        .insert(TilemapId(tilemap_entity));
}
