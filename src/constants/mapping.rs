use bevy::prelude::*;
use bevy_ecs_tilemap::map::TilemapTileSize;

pub const TILE: f32 = 16.;
pub const TILE_SIZE: TilemapTileSize = TilemapTileSize { x: TILE, y: TILE };
pub const CHUNK_SIZE: UVec2 = UVec2 { x: 6, y: 6 };
pub const CHUNK_SPAWN_RADIUS_X: i32 = 6;
pub const CHUNK_SPAWN_RADIUS_Y: i32 = 4;
pub const CHUNK_DESPAWN_RANGE_PX: f32 = (CHUNK_SIZE.x as f32 * CHUNK_SPAWN_RADIUS_X as f32 
                                       + CHUNK_SIZE.y as f32 * CHUNK_SPAWN_RADIUS_Y as f32) 
                                       * TILE_SIZE.x + 2.;
pub const RENDER_CHUNK_SIZE: UVec2 = UVec2 {
    x: CHUNK_SIZE.x * 2,
    y: CHUNK_SIZE.y * 2,
};
