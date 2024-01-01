use bevy::prelude::*;

use crate::components::flags::IsBot;
use crate::components::gpt::GPTAgent;

pub fn send_map_to_bot(
    mut bot_query: Query<&mut GPTAgent, With<IsBot>>
) {
    for agent in bot_query.iter_mut() {
        let is_empty = {
            if let Ok(queue) = agent.action_queue.try_lock() {
                queue.is_empty()
            } else {
                false
            }
        };

        if is_empty {
            agent.create_actions();
        }
    }
}
