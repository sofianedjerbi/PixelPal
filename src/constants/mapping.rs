use bevy::prelude::*;
use super::sprites::TILE_SIZE;


pub const CHUNK_SIZE: UVec2 = UVec2 { x: 8, y: 8 };
pub const CHUNK_SPAWN_RADIUS_X: i32 = 6;
pub const CHUNK_SPAWN_RADIUS_Y: i32 = 4;
pub const CHUNK_DESPAWN_RANGE_PX: f32 = (CHUNK_SIZE.x as f32 * CHUNK_SPAWN_RADIUS_X as f32 
                                       + CHUNK_SIZE.y as f32 * CHUNK_SPAWN_RADIUS_Y as f32) 
                                       * TILE_SIZE.x + 2.;
pub const RENDER_CHUNK_SIZE: UVec2 = UVec2 {
    x: CHUNK_SIZE.x * 2,
    y: CHUNK_SIZE.y * 2,
};
