use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

pub fn tilemap_setup(asset_server: Res<AssetServer>, mut commands: Commands) {
    // Load tilesheet texture and make a texture atlas from it
    let texture_handle = asset_server.load("tilesheet.png");
    // let texture_atlas = TextureAtlas::from_grid(texture_handle, vec2(8.0, 8.0), 4, 1, None, None);
    // let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let map_size = TilemapSize { x: 32, y: 32 };
    let tilemap_entity = commands.spawn_empty().id();
    let mut tile_storage = TileStorage::empty(map_size);

    let tile_size = TilemapTileSize { x: 8.0, y: 8.0 };
    let grid_size = tile_size.into();
    let map_type = TilemapType::default();

    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    texture_index: TileTextureIndex(0),
                    tilemap_id: TilemapId(tilemap_entity),
                    ..Default::default()
                })
                .id();
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    // Set up tilemap
    let tilemap_bundle = TilemapBundle {
        texture: TilemapTexture::Single(texture_handle.clone()),
        grid_size,
        map_type,
        tile_size,
        size: map_size,
        storage: tile_storage,
        transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
        ..Default::default()
    };

    // Spawn tilemap
    commands
        .entity(tilemap_entity)
        .insert(tilemap_bundle)
        .insert(TilemapId(tilemap_entity));
    println!("Spawned tilemap probably");
}

// fn add_tiles(mut commands: Commands, mut query: Query<&mut TileStorage, Added<TileStorage>>) {
//     for mut tile_storage in &mut query {
//         let pos = TilePos { x: 0, y: 0 };
//         println!("Actually setting tile");
//         let entity = commands
//             .spawn(TileBundle {
//                 position: pos,
//                 ..Default::default()
//             })
//             .id();
//
//         tile_storage.set(&pos, entity)
//     }
// }
