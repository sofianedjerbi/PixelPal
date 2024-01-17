use bevy::prelude::*;

use crate::components::gpt::GPTAgent;
use crate::constants::bot::*;

use super::mob::PlayerMobBundle;

/// Bundle for creating a GPT agent.
#[derive(Bundle)]
pub struct GptBundle {
    player_mob: PlayerMobBundle,
    agent: GPTAgent,
}

impl GptBundle {
    pub fn new(
        position: Vec2,
        texture: &Handle<Image>,
        texture_atlas: &mut ResMut<Assets<TextureAtlas>>,
        key: String,
        model: String,
        url: String,
    ) -> Option<Self> {
        GPTAgent::new(key, model, url).map(|mut agent| {
            agent.add_context(CONTEXT);
            agent.add_context(COMMANDS);
            GptBundle {
                player_mob: PlayerMobBundle::new(position, texture, texture_atlas),
                agent,
            }
        })
    }
}
