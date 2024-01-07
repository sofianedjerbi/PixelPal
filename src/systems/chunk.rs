use bevy::ecs::system::CommandQueue;
use bevy::prelude::*;
use bevy::log;
use bevy::tasks::AsyncComputeTaskPool;
use bevy::tasks::block_on;
use bevy_ecs_tilemap::prelude::*;
use futures_lite::future::poll_once;
use once_cell::sync::Lazy;
use rand::Rng;

use crate::bundles::map::*;
use crate::bundles::map::DataTileBundle;
use crate::components::map::*;
use crate::constants::generation::CACHE_SIZE;
use crate::constants::generation::LAYER_RANGE;
use crate::constants::generation::NOISE_ZOOM;
use crate::constants::generation::SAMPLE_NUMBER;
use crate::constants::map::*;
use crate::constants::textures::*;
use crate::util::noise::TiledNoise;
use crate::util::position::*;


static NOISE: Lazy<TiledNoise> = Lazy::new(|| {
    TiledNoise::new(
        0,
        LAYER_RANGE.to_vec(),
        NOISE_ZOOM,
        SAMPLE_NUMBER,
        CACHE_SIZE
    )
});

fn spawn_chunk_base(
    commands: &mut Commands,
    thread_pool: &AsyncComputeTaskPool,
    texture: TilemapTexture,
    chunk_pos: IVec2,
    all_chunks: &mut ResMut<ChunkMap>,
    player_chunk_map: &mut Mut<'_, ChunkMap>,
) {
    log::debug!("Spawning chunk: {}", chunk_pos);

    let layer_entity_0 = commands.spawn_empty().id();
    let layer_entity_1 = commands.spawn_empty().id();

    let task = thread_pool.spawn(async move {
        let mut command_queue = CommandQueue::default();

        command_queue.push(move |world: &mut World| {
            let mut tile_storage_0 = TileStorage::empty(
                CHUNK_SIZE.into()
            );
            let mut tile_storage_1 = TileStorage::empty(
                CHUNK_SIZE.into()
            );

            let base_x = chunk_pos.x as i32 * CHUNK_SIZE.x as i32;
            let base_y = chunk_pos.y as i32 * CHUNK_SIZE.y as i32;

            for x in 0..CHUNK_SIZE.x {
                for y in 0..CHUNK_SIZE.y {
                    let tile_pos = TilePos { x, y };
                    let level = NOISE.get_value(
                        base_x + x as i32 , 
                        base_y + y as i32
                    );
                    let mask = get_mask(
                        level,
                        x as i32 + base_x,
                        y as i32 + base_y
                    );
                    let id_0 = if mask == 0 {
                        get_random_tile_id(level)
                    } else {
                        get_random_tile_id(adjust_to_water_level(level))
                    };

                    let tile_bundle_0 = DataTileBundle {
                        tile: TileBundle {
                            position: tile_pos,
                            texture_index: TileTextureIndex(id_0),
                            tilemap_id: TilemapId(layer_entity_0),
                            ..Default::default()
                        },
                        level: ReliefLevel(level)
                    };

                    let tile_entity_0 = world.spawn(tile_bundle_0).id();
                    world.entity_mut(layer_entity_0).add_child(tile_entity_0);
                    tile_storage_0.set(&tile_pos, tile_entity_0);

                    if mask != 0 {
                        let id_1 = mask_to_id(mask, level);
                        let tile_bundle_1 = DataTileBundle {
                            tile: TileBundle {
                                position: tile_pos,
                                texture_index: TileTextureIndex(id_1),
                                tilemap_id: TilemapId(layer_entity_1),
                                ..Default::default()
                            },
                            level: ReliefLevel(level)
                        };
                        let tile_entity_1 = world.spawn(tile_bundle_1).id();
                        world.entity_mut(layer_entity_1).add_child(tile_entity_1);
                        tile_storage_1.set(&tile_pos, tile_entity_1);
                    }
                }
            }

            let transform_0 = Transform::from_xyz(
                base_x as f32 * TILE,
                base_y as f32 * TILE,
                0.0,
            );

            let transform_1 = Transform::from_xyz(
                base_x as f32 * TILE,
                base_y as f32 * TILE,
                1.0,
            );

            let layer0 = Layer::new(
                0,
                tile_storage_0,
                texture.clone(),
                transform_0
            );

            let layer1 = Layer::new(
                1,
                tile_storage_1,
                texture.clone(),
                transform_1
            );

            world.entity_mut(layer_entity_0).insert(layer0);
            world.entity_mut(layer_entity_1).insert(layer1);
        });

        command_queue
    });

    commands.spawn_empty().insert(ChunkTask(task));
    all_chunks.insert(chunk_pos, (layer_entity_0, layer_entity_1));
    player_chunk_map.insert(chunk_pos, (layer_entity_0, layer_entity_1));
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
        chunk_map.retain(|chunk_ipos, (layer0, layer1)| {
            let chunk_pos = chunk_pos_to_pixel_pos(chunk_ipos);
            let distance = transform.translation.xy()
                .distance_squared(chunk_pos);

            if distance < CHUNK_DESPAWN_RANGE_PX_SQUARED {
                true // Keep the chunk
            } else {
                log::debug!("Despawning chunk: {}", chunk_ipos);
                commands.entity(*layer0).despawn_recursive();
                commands.entity(*layer1).despawn_recursive();
                all_chunks.remove(chunk_ipos);
                false // Despawn the chunk
            }
        });
    }
}

pub fn fetch_chunk_tasks(
    mut commands: Commands,
    mut transform_tasks: Query<(Entity, &mut ChunkTask)>
) {
    for (entity, mut task) in &mut transform_tasks {
        if let Some(mut queue) = block_on(poll_once(&mut task.0)) {
            commands.add(move |world: &mut World| {
                queue.apply(world);
            });
            commands.entity(entity).despawn();
        }
    }
}

pub fn create_chunk_tasks(
    mut commands: Commands,
    mut all_chunks: ResMut<ChunkMap>,
    texture: Res<MainTilemapTexture>,
    mut loader_query: Query<(&Transform, &mut ChunkMap)>
) {
    let thread_pool = AsyncComputeTaskPool::get();
    for (
        transform,
        mut player_chunk_map
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
                    spawn_chunk_base(
                        &mut commands,
                        thread_pool,
                        texture.get(),
                        chunk_ipos,
                        &mut all_chunks,
                        &mut player_chunk_map
                    );
                }
            }
        }
    }
}

fn get_random_tile_id(level: u32) -> u32 {
    let random_number = rand::thread_rng().gen_range(0..=1000);
    let tile_probability_map = TEXTURE_RELIEF_IDS_MAP.get(&level).unwrap();
    let mut keys_less_than_random: Vec<&u32> = tile_probability_map.keys().filter(
        |&&key| key <= random_number
    ).collect();
    keys_less_than_random.sort();
    let key = keys_less_than_random.last().unwrap();
    *tile_probability_map.get(key).unwrap() + TEXTURE_ID_OFFSET_MAP[&level]
}


fn compare_relative_to_water(
    sample: u32,
    layer: u32
) -> bool {
    match adjust_to_water_level(layer) {
        n if n < layer => sample < layer,
        n if n > layer => sample > layer,
        _ => false
    }
}


fn get_mask(value: u32,x: i32, y: i32) -> u32 {
    let got_n = compare_relative_to_water(
        NOISE.get_value(x, y + 1), value
    );
    let got_s = compare_relative_to_water(
        NOISE.get_value(x, y - 1), value
    );
    let got_e = compare_relative_to_water(
        NOISE.get_value(x + 1, y), value
    );
    let got_w = compare_relative_to_water(
        NOISE.get_value(x - 1, y), value
    );
    let got_nw = compare_relative_to_water(
        NOISE.get_value(x - 1, y + 1), value
    );
    let got_ne = compare_relative_to_water(
        NOISE.get_value(x + 1, y + 1), value
    );
    let got_sw = compare_relative_to_water(
        NOISE.get_value(x - 1, y - 1), value
    );
    let got_se = compare_relative_to_water(
        NOISE.get_value(x + 1, y - 1), value
    );

    0b000_0_0_000 
        + if got_n { 0b010_0_0_000 } else { 0 }
        + if got_s { 0b000_0_0_010 } else { 0 }
        + if got_w { 0b000_1_0_000 } else { 0 }
        + if got_e { 0b000_0_1_000 } else { 0 }
        + if got_nw { 0b100_0_0_000 } else { 0 }
        + if got_ne { 0b001_0_0_000 } else { 0 }
        + if got_sw { 0b000_0_0_100 } else { 0 }
        + if got_se { 0b000_0_0_001 } else { 0 }
}

fn mask_to_id(mask: u32, value: u32) -> u32 {
    TEXTURE_CORNER_IDS_MAP[&mask] + TEXTURE_ID_OFFSET_MAP[&value]
}

fn adjust_to_water_level(n: u32) -> u32 {
    match n {
        n if n < WATER_LEVEL => n + 1,
        n if n > WATER_LEVEL => n - 1,
        _ => n,
    }
}
