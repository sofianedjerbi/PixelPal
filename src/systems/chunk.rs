use bevy::prelude::*;
use bevy::log;
use bevy_ecs_tilemap::prelude::*;

use crate::bundles::map::ChunkBundle;
use crate::bundles::map::DataTileBundle;
use crate::components::map::*;
use crate::constants::mapping::*;
use crate::util::noise::TiledNoise;
use crate::util::position::*;


fn spawn_chunk(
    commands: &mut Commands,
    asset_server: &AssetServer,
    chunk_pos: IVec2,
) -> Entity {
    log::debug!("Spawning chunk: {}", chunk_pos);
    let chunk_entity = commands.spawn_empty().id();
    let mut tile_storage = TileStorage::empty(CHUNK_SIZE.into());

    let base_x = chunk_pos.x as i32 * CHUNK_SIZE.x as i32;
    let base_y = chunk_pos.y as i32 * CHUNK_SIZE.y as i32;
    let noise = TiledNoise::new(0);

    for x in 0..CHUNK_SIZE.x {
        for y in 0..CHUNK_SIZE.y {
            let tile_pos = TilePos { x, y };
            let value = noise.get_value(base_x + x as i32 , base_y + y as i32);
            let tile_bundle = DataTileBundle {
                tile: TileBundle {
                    position: tile_pos,
                    texture_index: TileTextureIndex(value),
                    tilemap_id: TilemapId(chunk_entity),
                    ..Default::default()
                },
                level: ReliefLevel(value)
            };
            let tile_entity = commands.spawn(tile_bundle).id();
            commands.entity(chunk_entity).add_child(tile_entity);
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    let transform = Transform::from_xyz(
        base_x as f32 * TILE,
        base_y as f32 * TILE,
        0.0,
    );

    let texture_handle: Handle<Image> = asset_server.load("tileset/environment/debug.png");

    let chunk = ChunkBundle::new(
        tile_storage,
        TilemapTexture::Single(texture_handle),
        transform
    );
    commands.entity(chunk_entity).insert(chunk);

    chunk_entity
}

pub fn handle_chunk_despawning(
    mut commands: Commands,
    mut all_chunks: ResMut<ChunkMap>,
    mut loader_query: Query<(&Transform, &mut ChunkMap)>
) {
    for (
        transform,
        mut chunk_map
    ) in loader_query.iter_mut() {
        chunk_map.retain(|chunk_ipos, entity| {
            let chunk_pos = chunk_pos_to_pixel_pos(chunk_ipos);
            let distance = transform.translation.xy()
                .distance_squared(chunk_pos);

            if distance < CHUNK_DESPAWN_RANGE_PX_SQUARED {
                true // Keep the chunk
            } else {
                commands.entity(*entity).despawn_recursive();
                all_chunks.remove(chunk_ipos);
                log::debug!("Despawning chunk: {}", chunk_ipos);
                false // Remove the chunk
            }
        });
    }
}

pub fn handle_chunk_spawning(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut all_chunks: ResMut<ChunkMap>,
    mut loader_query: Query<(&Transform, &mut ChunkMap)>
) {
    for (
        transform,
        mut chunk_map
    ) in loader_query.iter_mut() {
        let camera_chunk_pos = pixel_pos_to_chunk_pos(
            &transform.translation.xy()
        );
        for y in (camera_chunk_pos.y - CHUNK_SPAWN_RADIUS_Y)
                    ..(camera_chunk_pos.y + CHUNK_SPAWN_RADIUS_Y) {
            for x in (camera_chunk_pos.x - CHUNK_SPAWN_RADIUS_X)
                        ..(camera_chunk_pos.x + CHUNK_SPAWN_RADIUS_X) {
                let chunk_ipos = IVec2::new(x, y);
                if !all_chunks.contains_key(&chunk_ipos) {
                    let chunk_entity = spawn_chunk(
                        &mut commands,
                        &asset_server,
                        chunk_ipos
                    );
                    all_chunks.insert(chunk_ipos, chunk_entity);
                    chunk_map.insert(chunk_ipos, chunk_entity);
                }
            }
        }
    }
}
