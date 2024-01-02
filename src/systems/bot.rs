use bevy::{prelude::*, log};
use bevy_ecs_tilemap::tiles::{TileStorage, TilePos};

use crate::components::flags::*;
use crate::components::gpt::GPTAgent;
use crate::components::map::*;
use crate::constants::bot::BOT_VIEW_DISTANCE;
use crate::constants::sprites::PLAYER_SPRITE_SIZE;
use crate::util::position::*;

pub fn send_map_to_bot(
    chunk_map: Res<ChunkMap>,
    chunk_query: Query<&TileStorage>,
    tile_query: Query<&ReliefLevel>,
    bot_query: Query<(&Transform, &GPTAgent), With<IsBot>>,
    user_query: Query<&Transform, With<IsUser>>
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

        if agent.is_busy() {
            return;
        }

        let bot_tile_pos = pixel_pos_to_tile_pos_player(
            &transform.translation.xy(),
            PLAYER_SPRITE_SIZE.y / 2.
        );

        let mut map = Vec::new();

        // Write tiles relief
        for y in ((bot_tile_pos.y - BOT_VIEW_DISTANCE)
                     ..(bot_tile_pos.y + BOT_VIEW_DISTANCE + 1)).rev() {
            for x in (bot_tile_pos.x - BOT_VIEW_DISTANCE)
                        ..(bot_tile_pos.x + BOT_VIEW_DISTANCE + 1) {
                let tile_pos = IVec2::new(x, y);

                let chunk_pos = tile_pos_to_chunk_pos(&tile_pos);
                let relative_tile_pos = relative_tile_pos(&tile_pos);
                let relative_tile_pos = TilePos {
                    x: relative_tile_pos.x as u32,
                    y: relative_tile_pos.y as u32
                };

                if let Some(chunk_entity) = chunk_map.get(&chunk_pos) {
                    match chunk_query.get(*chunk_entity) {
                        Ok(tile_storage) => {
                            if let Some(tile_entity) = tile_storage.get(&relative_tile_pos) {
                                match tile_query.get(tile_entity) {
                                    Ok(relief_level) => map.push(std::char::from_digit(**relief_level, 10).unwrap()),
                                    Err(e) => {
                                        log::error!("Failed to get relief level: {}", e);
                                        map.push('a');
                                    },
                                }
                            } else {
                                log::error!("No tile entity found at the given position");
                                map.push('b');
                            }
                        },
                        Err(e) => {
                            log::error!("Failed to get tile storage: {}", e);
                            map.push('c');
                        }
                    }
                } else {
                    log::error!("No chunk entity found at the given position");
                    map.push('d');
                }
            }
            map.push('\n');
        }

        // Write bot position
        map[BOT_VIEW_DISTANCE as usize * (BOT_VIEW_DISTANCE * 2 + 3) as usize] = 'U';

        // Write user position
        for transform in user_query.iter() {
            let user_tile_pos = pixel_pos_to_tile_pos_player(
                &transform.translation.xy(),
                PLAYER_SPRITE_SIZE.y / 2.
            );

            let mut relative_position = user_tile_pos - bot_tile_pos;
            log::error!("relative_position: {}", relative_position);
            if relative_position.x.abs() <= BOT_VIEW_DISTANCE
            && relative_position.y.abs() <= BOT_VIEW_DISTANCE {
                relative_position.y = - relative_position.y;
                let index = relative_position.x + BOT_VIEW_DISTANCE 
                               + (relative_position.y + BOT_VIEW_DISTANCE) 
                               * (BOT_VIEW_DISTANCE * 2 + 2);
                log::error!("Index: {}", index);
                map[index as usize] = 'P';
            }
        }

        //log::error!("{}", &map.iter().collect::<String>());
        agent.create_actions_with_extra_context(&map.iter().collect::<String>());
    }
}
