use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use crate::constants::mapping::*;


#[derive(Bundle)]
pub struct ChunkBundle {
    pub tilemap: TilemapBundle,
}

impl ChunkBundle {
    pub fn new(
        storage: TileStorage,
        texture: TilemapTexture,
        transform: Transform
    ) -> Self {
        Self {
            tilemap: TilemapBundle {
                grid_size: TILE_SIZE.into(),
                size: CHUNK_SIZE.into(),
                storage: storage,
                texture: texture,
                tile_size: TILE_SIZE,
                transform: transform,
                ..Default::default()
            }
        }
    }
}
