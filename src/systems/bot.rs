use bevy::prelude::*;

use crate::components::characters::*;
use crate::components::gpt::GPTAgent;
use crate::components::textures::TilesetOffset;
use crate::constants::bot::BOT_VIEW_DISTANCE;
use crate::util::position::*;

/// Processes bot behavior based on the environment and user positions.
///
/// This function checks the surroundings of each bot and updates its actions
/// based on the relative position of the user. It leverages the GPTAgent to
/// create actions for the bot based on the current game context.
///
/// # Parameters
/// - `bot_query`: Query to access bot characters and their properties.
/// - `user_query`: Query to access user characters and their properties.
pub fn query_bot(
    bot_query: Query<(&Transform, &TilesetOffset, &GPTAgent), With<IsBot>>,
    user_query: Query<(&Transform, &TilesetOffset), With<IsUser>>,
) {
    for (transform, offset, agent) in bot_query.iter() {
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

        let bot_tile_pos = player_tile_pos(transform, offset);

        let mut map = String::from("Environment:\n");

        // Write user position
        for (transform, offset) in user_query.iter() {
            let user_tile_pos = player_tile_pos(transform, offset);
            let relative_position = user_tile_pos - bot_tile_pos;
            if relative_position.x.abs() > BOT_VIEW_DISTANCE
                || relative_position.y.abs() > BOT_VIEW_DISTANCE
            {
                continue;
            }

            let horizontal_direction = if relative_position.x > 0 {
                "right"
            } else {
                "left"
            };
            let vertical_direction = if relative_position.y > 0 {
                "up"
            } else {
                "down"
            };

            map.push_str(&format!(
                "Player is {} to your {} and {} to your {}",
                relative_position.x.abs(),
                horizontal_direction,
                relative_position.y.abs(),
                vertical_direction
            ));
        }

        agent.create_actions_with_extra_context(&map);
    }
}
