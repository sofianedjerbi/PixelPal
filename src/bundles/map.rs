use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use crate::constants::{mapping::*, sprites::TILE_SIZE};


#[derive(Bundle)]
pub struct Map {
    pub tilemap: TilemapBundle,
}

impl Map {
    pub fn new(
        storage: TileStorage,
        texture: TilemapTexture,
        transform: Transform
    ) -> Self {
        Map {
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
