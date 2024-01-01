use bevy::prelude::*;
use bevy::log;
use bevy_ecs_tilemap::prelude::*;
use crate::bundles::map::ChunkBundle;
use crate::components::mapping::*;
use crate::constants::mapping::*;
use crate::generation::noise::TiledNoise;


fn spawn_chunk(
    commands: &mut Commands,
    asset_server: &AssetServer,
    chunk_list: &mut ChunkList,
    chunk_pos: IVec2,
) {
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
            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    texture_index: TileTextureIndex(value),
                    tilemap_id: TilemapId(chunk_entity),
                    ..Default::default()
                }).id();
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

    chunk_list.list.insert(chunk_pos, chunk_entity);

    commands.entity(chunk_entity).insert(chunk);
}

pub fn handle_chunk_despawning(
    mut commands: Commands,
    mut all_chunks: ResMut<IndexChunkList>,
    mut loader_query: Query<(&Transform, &mut ChunkList)>
) {
    for (
        transform,
        mut chunk_list
    ) in loader_query.iter_mut() {
        chunk_list.list.retain(|chunk_ipos, entity| {
            let chunk_pos = chunk_pos_to_camera_pos(chunk_ipos);
            let distance = transform.translation.xy()
                .distance_squared(chunk_pos);

            if distance < CHUNK_DESPAWN_RANGE_PX_SQUARED {
                true // Keep the chunk
            } else {
                commands.entity(*entity).despawn_recursive();
                all_chunks.list.remove(chunk_ipos);
                log::debug!("Despawning chunk: {}", chunk_ipos);
                false // Remove the chunk
            }
        });
    }
}

pub fn handle_chunk_spawning(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut all_chunks: ResMut<IndexChunkList>,
    mut loader_query: Query<(&Transform, &mut ChunkList)>
) {
    for (
        transform,
        mut chunk_list
    ) in loader_query.iter_mut() {
        let camera_chunk_pos = camera_pos_to_chunk_pos(
            &transform.translation.xy()
        );

        for y in (camera_chunk_pos.y - CHUNK_SPAWN_RADIUS_Y)
                    ..(camera_chunk_pos.y + CHUNK_SPAWN_RADIUS_Y) {
            for x in (camera_chunk_pos.x - CHUNK_SPAWN_RADIUS_X)
                        ..(camera_chunk_pos.x + CHUNK_SPAWN_RADIUS_X) {
                let chunk_ipos = IVec2::new(x, y);
                if !all_chunks.list.contains(&chunk_ipos) {
                    all_chunks.list.insert(chunk_ipos);
                    spawn_chunk(
                        &mut commands,
                        &asset_server,
                        &mut chunk_list,
                        chunk_ipos
                    );
                }
            }
        }
    }
}

pub fn camera_pos_to_chunk_pos(camera_pos: &Vec2) -> IVec2 {
    let camera_pos = camera_pos.as_ivec2();
    let chunk_size: IVec2 = IVec2::new(CHUNK_SIZE.x as i32, CHUNK_SIZE.y as i32);
    let tile_size: IVec2 = IVec2::new(TILE as i32, TILE as i32);
    camera_pos / (chunk_size * tile_size)
}

pub fn chunk_pos_to_camera_pos(chunk_pos: &IVec2) -> Vec2 {
    let chunk_size: IVec2 = CHUNK_SIZE.as_ivec2();
    let tile_size: IVec2 = IVec2::new(TILE as i32, TILE as i32);
    let camera_pos = *chunk_pos * chunk_size * tile_size;
    camera_pos.as_vec2()
}
