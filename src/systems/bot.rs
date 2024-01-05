use bevy::prelude::*;

use crate::components::characters::*;
use crate::components::gpt::GPTAgent;
use crate::constants::bot::BOT_VIEW_DISTANCE;
use crate::constants::sprites::PLAYER_SPRITE_SIZE;
use crate::util::position::*;

pub fn send_map_to_bot(
    //chunk_map: Res<ChunkMap>,
    //chunk_query: Query<&TileStorage>,
    //tile_query: Query<&ReliefLevel>,
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

        if !is_empty || agent.is_busy() {
            return;
        }

        let bot_tile_pos = pixel_pos_to_tile_pos_player(
            &transform.translation.xy(),
            PLAYER_SPRITE_SIZE.y / 2.
        );

        let mut map = String::from("Environment:\n");

        // Write user position
        for transform in user_query.iter() {
            let user_tile_pos = pixel_pos_to_tile_pos_player(
                &transform.translation.xy(),
                PLAYER_SPRITE_SIZE.y / 2.
            );
            let relative_position = user_tile_pos - bot_tile_pos;
            if relative_position.x.abs() > BOT_VIEW_DISTANCE ||
               relative_position.y.abs() > BOT_VIEW_DISTANCE {
                continue;
            }
            
            let horizontal_direction = if relative_position.x > 0 { "right" } else { "left" };
            let vertical_direction = if relative_position.y > 0 { "up" } else { "down" };

            map.push_str(&format!("Player is {} to your {} and {} to your {}",
                relative_position.x.abs(),
                horizontal_direction,
                relative_position.y.abs(),
                vertical_direction
            ));
        }

        //log::error!("{}", &map.iter().collect::<String>());
        agent.create_actions_with_extra_context(&map);
    }
}
