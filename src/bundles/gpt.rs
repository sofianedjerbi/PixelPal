use bevy::prelude::*;

use crate::components::gpt::GPTAgent;
use crate::constants::bot::{COMMANDS, CONTEXT};

use super::player::PlayerBundle;

#[derive(Bundle)]
pub struct GptBundle {
    player: PlayerBundle,
    agent: GPTAgent
}

impl GptBundle {
    pub fn new(
        position: Vec2,
        asset_server: &Res<AssetServer>,
        textures: &mut ResMut<Assets<TextureAtlas>>,
        key: &str
    ) -> Option<Self> {
        match GPTAgent::new(key) {
            None => None,
            Some(mut agent) => {
                agent.add_context(CONTEXT);
                agent.add_context(COMMANDS);
                Some(GptBundle {
                    player: PlayerBundle::new(position, asset_server, textures),
                    agent
                })
            }
        }
    }
}

