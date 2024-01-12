use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::TilePos;

use crate::{constants::map::*, components::textures::TilesetOffset};


/// Converts a pixel position to chunk position.
/// 
/// # Parameters
/// - `pixel_pos`: The pixel position to be converted.
/// 
/// # Returns
/// The corresponding chunk position as `IVec2`.
pub fn pixel_pos_to_chunk_pos(pixel_pos: &Vec2) -> IVec2 {
    let pixel_pos = pixel_pos.as_ivec2();
    let chunk_size: IVec2 = IVec2::new(CHUNK_SIZE.x as i32, CHUNK_SIZE.y as i32);
    let tile_size: IVec2 = IVec2::new(TILE as i32, TILE as i32);
    pixel_pos / (chunk_size * tile_size)
}

/// Converts a chunk position to pixel position.
/// 
/// # Parameters
/// - `chunk_pos`: The chunk position to be converted.
/// 
/// # Returns
/// The corresponding pixel position as `Vec2`.
pub fn chunk_pos_to_pixel_pos(chunk_pos: &IVec2) -> Vec2 {
    let chunk_size: IVec2 = CHUNK_SIZE.as_ivec2();
    let tile_size: IVec2 = IVec2::new(TILE as i32, TILE as i32);
    let pixel_pos = *chunk_pos * chunk_size * tile_size;
    pixel_pos.as_vec2()
}

/// Calculates the tile position of a player based on their transform and offset.
/// 
/// # Parameters
/// - `transform`: The transform component of the player.
/// - `offset`: The tileset offset for the player.
/// 
/// # Returns
/// The tile position as `IVec2`.
pub fn player_tile_pos(tranform: &Transform, offset: &TilesetOffset) -> IVec2 {
    pixel_pos_to_tile_pos(&(
        tranform.translation.xy() 
        - **offset
    ))
}

/// Converts a pixel position to a tile position.
/// 
/// # Parameters
/// - `pixel_pos`: The pixel position to be converted.
/// 
/// # Returns
/// The corresponding tile position as `IVec2`.
pub fn pixel_pos_to_tile_pos(pixel_pos: &Vec2) -> IVec2 {
    (*pixel_pos * 1. / TILE).as_ivec2()
}

/// Converts a tile position to a chunk position.
/// 
/// # Parameters
/// - `tile_pos`: The tile position to be converted.
/// 
/// # Returns
/// The corresponding chunk position as `IVec2`.
pub fn tile_pos_to_chunk_pos(tile_pos: &IVec2) -> IVec2 {
    tile_pos.div_euclid(CHUNK_SIZE.as_ivec2())
}

/// Calculates the relative tile position within its chunk.
/// 
/// # Parameters
/// - `tile_pos`: The tile position to be converted.
/// 
/// # Returns
/// The relative position as `TilePos`.
pub fn relative_tile_pos(tile_pos: &IVec2) -> TilePos {
    let position = tile_pos.rem_euclid(CHUNK_SIZE.as_ivec2()).as_uvec2();
    TilePos {
        x: position.x,
        y: position.y
    }
}
