use std::sync::Arc;

use bevy::ecs::system::CommandQueue;
use bevy::log;
use bevy::prelude::*;
use bevy::tasks::AsyncComputeTaskPool;
use bevy_ecs_tilemap::prelude::*;
use once_cell::sync::Lazy;

use crate::bundles::map::DataTileBundle;
use crate::bundles::map::*;
use crate::components::map::*;
use crate::constants::generation::CACHE_SIZE;
use crate::constants::generation::LAYER_RANGE;
use crate::constants::generation::NOISE_ZOOM;
use crate::constants::generation::SAMPLE_NUMBER;
use crate::constants::map::*;
use crate::constants::tileset::*;
use crate::util::noise::TiledNoise;
use crate::util::position::*;
use crate::util::tile::*;

static NOISE: Lazy<TiledNoise> =
    Lazy::new(|| TiledNoise::new(0, &LAYER_RANGE, NOISE_ZOOM, SAMPLE_NUMBER, CACHE_SIZE));

// Compact layer definition
struct LayerConfig<'a> {
    layer_index: u32,
    tile_storage: &'a TileStorage,
    texture: &'a Arc<TilemapTexture>,
    base_x: i32,
    base_y: i32,
    z_position: f32,
}

// Compact tile definition
struct LayeredTileConfig<'a> {
    tile_storage_0: &'a mut TileStorage,
    tile_storage_1: &'a mut TileStorage,
    layer_entity_0: Entity,
    layer_entity_1: Entity,
    base_x: i32,
    base_y: i32,
    x: u32,
    y: u32,
}

/// Handles despawning of chunks that are out of range.
///
/// This function iterates through chunks and despawns those that are beyond the
/// specified despawn range from the player's current position.
///
/// # Parameters
/// - `commands`: Commands for entity manipulation.
/// - `all_chunks`: Resource containing all chunk data.
/// - `loader_query`: Query for accessing chunk loader transforms.
pub fn handle_chunk_despawning(
    mut commands: Commands,
    mut all_chunks: ResMut<ChunkMap>,
    mut loader_query: Query<(&Transform, &mut ChunkMap)>,
) {
    for (transform, mut chunk_map) in loader_query.iter_mut() {
        chunk_map.retain(|chunk_ipos, (layer0, layer1)| {
            let chunk_pos = chunk_pos_to_pixel_pos(chunk_ipos);
            let distance = transform.translation.xy().distance_squared(chunk_pos);

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

/// Creates tasks for generating new chunks around the players.
///
/// This function spawns new chunks within the spawn radius around the player.s
/// It uses an asynchronous compute pool for chunk generation tasks.
///
/// # Parameters
/// - `commands`: Commands for entity manipulation.
/// - `all_chunks`: Resource containing all chunk data.
/// - `texture`: Resource of the main tilemap texture.
/// - `loader_query`: Query for accessing chunk loader transforms.
pub fn create_chunk_tasks(
    mut commands: Commands,
    mut all_chunks: ResMut<ChunkMap>,
    channel: Res<ChunkSpawningChannel>,
    texture: Res<MainTilemapTexture>,
    mut loader_query: Query<(&Transform, &mut ChunkMap)>,
) {
    let thread_pool = AsyncComputeTaskPool::get();
    for (transform, mut player_chunk_map) in loader_query.iter_mut() {
        let camera_chunk_pos = pixel_pos_to_chunk_pos(&transform.translation.xy());
        for y in
            (camera_chunk_pos.y - CHUNK_SPAWN_RADIUS_Y)..(camera_chunk_pos.y + CHUNK_SPAWN_RADIUS_Y)
        {
            for x in (camera_chunk_pos.x - CHUNK_SPAWN_RADIUS_X)
                ..(camera_chunk_pos.x + CHUNK_SPAWN_RADIUS_X)
            {
                let chunk_ipos = IVec2::new(x, y);
                if !all_chunks.contains_key(&chunk_ipos) {
                    spawn_chunk_base(
                        &mut commands,
                        thread_pool,
                        &channel,
                        chunk_ipos,
                        &mut all_chunks,
                        &mut player_chunk_map,
                        texture.clone_arc(),
                    );
                }
            }
        }
    }
}

/// Fetches and applies completed chunk generation tasks.
///
/// This function checks for completed asynchronous tasks for chunk generation
/// and applies them to the world state.
///
/// # Parameters
/// - `commands`: Commands for entity manipulation.
/// - `transform_tasks`: Query for accessing chunk task components.
pub fn fetch_chunk_tasks(mut commands: Commands, mut channel: ResMut<ChunkSpawningChannel>) {
    while let Ok(mut queue) = channel.receiver.try_recv() {
        commands.add(move |world: &mut World| {
            queue.apply(world);
        });
    }
}

/// Spawns a new chunk and adds it to the world.
fn spawn_chunk_base(
    commands: &mut Commands,
    thread_pool: &AsyncComputeTaskPool,
    channel: &Res<ChunkSpawningChannel>,
    chunk_pos: IVec2,
    all_chunks: &mut ResMut<ChunkMap>,
    player_chunk_map: &mut Mut<'_, ChunkMap>,
    texture: Arc<TilemapTexture>,
) {
    log::debug!("Spawning chunk: {}", chunk_pos);

    let layer_entity_0 = commands.spawn_empty().id();
    let layer_entity_1 = commands.spawn_empty().id();

    create_chunk_task(
        thread_pool,
        channel,
        chunk_pos,
        layer_entity_0,
        layer_entity_1,
        texture,
    );

    all_chunks.insert(chunk_pos, (layer_entity_0, layer_entity_1));
    player_chunk_map.insert(chunk_pos, (layer_entity_0, layer_entity_1));
}

/// Creates an asynchronous task for generating a chunk.
fn create_chunk_task(
    thread_pool: &AsyncComputeTaskPool,
    channel: &Res<ChunkSpawningChannel>,
    chunk_pos: IVec2,
    layer_entity_0: Entity,
    layer_entity_1: Entity,
    texture: Arc<TilemapTexture>,
) {
    let sender = channel.sender.clone();
    thread_pool
        .spawn(async move {
            let mut command_queue = CommandQueue::default();
            populate_command_queue(
                &mut command_queue,
                chunk_pos,
                layer_entity_0,
                layer_entity_1,
                texture,
            );
            match sender.send(command_queue).await {
                Ok(_) => log::debug!("Chunk {} {} successfully sent.", chunk_pos.x, chunk_pos.y),
                Err(e) => log::error!(
                    "Failed to send chunk {} {}: {:?}",
                    chunk_pos.x,
                    chunk_pos.y,
                    e
                ),
            }
        })
        .detach();
}

/// Populates a command queue with tile setup commands for a chunk.
fn populate_command_queue(
    command_queue: &mut CommandQueue,
    chunk_pos: IVec2,
    layer_entity_0: Entity,
    layer_entity_1: Entity,
    texture: Arc<TilemapTexture>,
) {
    command_queue.push(move |world: &mut World| {
        let mut tile_storage_0 = TileStorage::empty(CHUNK_SIZE.into());
        let mut tile_storage_1 = TileStorage::empty(CHUNK_SIZE.into());
        let base_x = chunk_pos.x * CHUNK_SIZE.x as i32;
        let base_y = chunk_pos.y * CHUNK_SIZE.y as i32;

        for x in 0..CHUNK_SIZE.x {
            for y in 0..CHUNK_SIZE.y {
                let layered_tile_setup_0 = LayeredTileConfig {
                    tile_storage_0: &mut tile_storage_0,
                    tile_storage_1: &mut tile_storage_1,
                    layer_entity_0,
                    layer_entity_1,
                    base_x,
                    base_y,
                    x,
                    y,
                };

                setup_tile(world, layered_tile_setup_0);
            }
        }

        let config_0 = LayerConfig {
            layer_index: 0,
            tile_storage: &tile_storage_0,
            texture: &texture,
            base_x,
            base_y,
            z_position: 0.0,
        };

        let config_1 = LayerConfig {
            layer_index: 1,
            tile_storage: &tile_storage_1,
            texture: &texture,
            base_x,
            base_y,
            z_position: 1.0,
        };

        add_layer_to_world(world, layer_entity_0, config_0);
        add_layer_to_world(world, layer_entity_1, config_1);
    });
}

/// Sets up individual tiles within a chunk.
fn setup_tile(world: &mut World, tile_config: LayeredTileConfig) {
    let LayeredTileConfig {
        tile_storage_0,
        tile_storage_1,
        layer_entity_0,
        layer_entity_1,
        base_x,
        base_y,
        x,
        y,
    } = tile_config;
    let pos_x = base_x + x as i32;
    let pos_y = base_y + y as i32;
    let tile_pos = TilePos { x, y };
    let level = NOISE.get_value(pos_x, pos_y);
    let mask = NOISE.get_mask(level, pos_x, pos_y);
    let is_edge = mask != 0;
    let id_0 = if !is_edge {
        get_random_tile_id(level)
    } else {
        get_random_tile_id(level - 1)
    };

    let tile_bundle_0 = DataTileBundle {
        tile: TileBundle {
            position: tile_pos,
            texture_index: TileTextureIndex(id_0),
            tilemap_id: TilemapId(layer_entity_0),
            ..Default::default()
        },
        level: ReliefLevel(level),
    };

    let tile_entity_0 = world.spawn(tile_bundle_0).id();
    world.entity_mut(layer_entity_0).add_child(tile_entity_0);
    tile_storage_0.set(&tile_pos, tile_entity_0);

    if let Some(animation) = TEXTURE_ANIMATION_MAP.lookup(&(level, id_0)) {
        world.entity_mut(tile_entity_0).insert(AnimatedTile {
            start: animation.start,
            end: animation.end,
            speed: animation.speed,
        });
    }

    if is_edge {
        let id_1 = mask_to_id(mask, level);
        let tile_bundle_1 = DataTileBundle {
            tile: TileBundle {
                position: tile_pos,
                texture_index: TileTextureIndex(id_1),
                tilemap_id: TilemapId(layer_entity_1),
                ..Default::default()
            },
            level: ReliefLevel(level),
        };
        let tile_entity_1 = world.spawn(tile_bundle_1).id();
        world.entity_mut(layer_entity_1).add_child(tile_entity_1);
        tile_storage_1.set(&tile_pos, tile_entity_1);
    }
}

/// Adds a layer containing tiles to the world.
fn add_layer_to_world(world: &mut World, layer_entity: Entity, layer_config: LayerConfig) {
    let LayerConfig {
        layer_index,
        tile_storage,
        texture,
        base_x,
        base_y,
        z_position,
    } = layer_config;

    let transform = Transform::from_xyz(base_x as f32 * TILE, base_y as f32 * TILE, z_position);

    let layer = Layer::new(
        layer_index,
        tile_storage.clone(),
        texture.clone_weak(),
        transform,
    );

    world.entity_mut(layer_entity).insert(layer);
}
