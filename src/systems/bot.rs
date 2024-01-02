use bevy::{prelude::*, log};
use bevy_ecs_tilemap::tiles::{TileStorage, TilePos};

use crate::components::flags::IsBot;
use crate::components::gpt::GPTAgent;
use crate::components::map::*;
use crate::constants::bot::BOT_VIEW_DISTANCE;
use crate::constants::sprites::PLAYER_SPRITE_SIZE;
use crate::util::position::*;

pub fn send_map_to_bot(
    chunk_map: Res<ChunkMap>,
    chunk_query: Query<&TileStorage>,
    tile_query: Query<&ReliefLevel>,
    bot_query: Query<(&Transform, &GPTAgent), With<IsBot>>
) {
    for (transform, agent) in bot_query.iter() {
        let is_empty = {
            if let Ok(queue) = agent.action_queue.try_lock() {
                queue.is_empty()
            } else {
                false
            }
        };

        if !is_empty {
            return;
        }
        

        let player_tile_pos = pixel_pos_to_tile_pos_player(
            &transform.translation.xy(),
            PLAYER_SPRITE_SIZE.y / 2.
        );

        let mut map = String::from(format!("{}x{}\n", BOT_VIEW_DISTANCE, BOT_VIEW_DISTANCE));

        for y in (player_tile_pos.y - BOT_VIEW_DISTANCE)
                    ..(player_tile_pos.y + BOT_VIEW_DISTANCE) {
            for x in (player_tile_pos.x - BOT_VIEW_DISTANCE)
                        ..(player_tile_pos.x + BOT_VIEW_DISTANCE) {
                let tile_pos = IVec2::new(x, y);

                if tile_pos == player_tile_pos {
                    map.push('X');
                    continue;
                }

                let chunk_pos = tile_pos_to_chunk_pos(&tile_pos);
                let relative_tile_pos = relative_tile_pos(&tile_pos);
                let relative_tile_pos = TilePos {
                    x: relative_tile_pos.x as u32,
                    y: relative_tile_pos.y as u32
                };
                log::error!("{:?}:{:?}:{:?}", tile_pos, relative_tile_pos, chunk_pos);

                if let Some(chunk_entity) = chunk_map.get(&chunk_pos) {
                    if let Ok(tile_storage) = chunk_query.get(*chunk_entity) {
                        if let Some(tile_entity) = tile_storage.get(&relative_tile_pos) {
                            if let Ok(relief_level) = tile_query.get(tile_entity) {
                                map.push_str(&format!("{}", relief_level));
                            } else {
                                map.push_str("?");
                            }
                        } else {
                            map.push_str("?");
                        }
                    } else {
                        map.push_str(&"?".repeat(BOT_VIEW_DISTANCE as usize));
                        map.push_str("\n");
                    }
                } else {
                    map.push_str(&"?".repeat(BOT_VIEW_DISTANCE as usize));
                    map.push_str("\n");
                }
            }
            map.push_str("\n");
        }
        agent.create_actions_with_extra_context(&map);
    }
}
