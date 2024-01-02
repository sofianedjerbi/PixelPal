use bevy::prelude::*;

use crate::constants::mapping::*;


pub fn pixel_pos_to_chunk_pos(pixel_pos: &Vec2) -> IVec2 {
    let pixel_pos = pixel_pos.as_ivec2();
    let chunk_size: IVec2 = IVec2::new(CHUNK_SIZE.x as i32, CHUNK_SIZE.y as i32);
    let tile_size: IVec2 = IVec2::new(TILE as i32, TILE as i32);
    pixel_pos / (chunk_size * tile_size)
}

pub fn pixel_pos_to_chunk_pos_player(pixel_pos: &Vec2, sprite_offset: f32) -> IVec2 {
    let aligned_pixel_pos = Vec2::new(pixel_pos.x, pixel_pos.y - sprite_offset);
    let pixel_pos = aligned_pixel_pos.as_ivec2();
    let chunk_size: IVec2 = IVec2::new(CHUNK_SIZE.x as i32, CHUNK_SIZE.y as i32);
    let tile_size: IVec2 = IVec2::new(TILE as i32, TILE as i32);
    pixel_pos / (chunk_size * tile_size)
}


pub fn chunk_pos_to_pixel_pos(chunk_pos: &IVec2) -> Vec2 {
    let chunk_size: IVec2 = CHUNK_SIZE.as_ivec2();
    let tile_size: IVec2 = IVec2::new(TILE as i32, TILE as i32);
    let pixel_pos = *chunk_pos * chunk_size * tile_size;
    pixel_pos.as_vec2()
}

pub fn pixel_pos_to_tile_pos_player(pixel_pos: &Vec2, sprite_offset: f32) -> IVec2 {
    IVec2::new(
        (pixel_pos.x / TILE).round() as i32,
        ((pixel_pos.y - sprite_offset) / TILE).round() as i32,
    )
}

pub fn tile_pos_to_chunk_pos(tile_pos: &IVec2) -> IVec2 {
    tile_pos.div_euclid(CHUNK_SIZE.as_ivec2())
}

pub fn relative_tile_pos(tile_pos: &IVec2) -> IVec2 {
    tile_pos.rem_euclid(CHUNK_SIZE.as_ivec2())
}
