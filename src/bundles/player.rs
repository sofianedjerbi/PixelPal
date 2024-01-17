use bevy::prelude::*;

use super::mob::PlayerMobBundle;

#[derive(Bundle)]
pub struct PlayerBundle {
    player_mob: PlayerMobBundle,
}

impl PlayerBundle {
    /// Creates a new player mob bundle with the specified parameters.
    pub fn new(
        position: Vec2,
        texture: &Handle<Image>,
        texture_atlas: &mut ResMut<Assets<TextureAtlas>>,
    ) -> Self {
        Self {
            player_mob: PlayerMobBundle::new(position, texture, texture_atlas),
        }
    }
}
