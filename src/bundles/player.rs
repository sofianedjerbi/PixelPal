use bevy::prelude::*;
use bevy::render::texture;
use crate::components::action::*;
use crate::components::animation::*;
use crate::components::characters::*;
use crate::components::map::ChunkMap;
use crate::components::textures::TilesetOffset;
use crate::constants::action::PLAYER_ACTION_DEFAULT;
use crate::constants::action::PLAYER_ACTION_DURATION_MAP;
use crate::constants::characters::*;
use crate::constants::map::TILE;
use crate::constants::sprites::*;

use super::animation::AnimationBundle;


#[derive(Bundle)]
pub struct PlayerBundle {
    pub busy: Busy,
    pub health: Health,
    pub current_action: Action,
    pub action_timer: ActionTimer,
    pub action_duration: ActionDurationPHF,
    pub chunk_map: ChunkMap,
    pub animation: AnimationBundle,
    pub animation_frames: ActionAnimationMap,
    pub offset: TilesetOffset
}

impl PlayerBundle {
    pub fn new(
        position: Vec2,
        texture: Handle<Image>,
        texture_atlas: &mut ResMut<Assets<TextureAtlas>>
    ) -> Self {
        Self {
            busy: Busy(false),
            health: PLAYER_HEALTH,
            current_action: PLAYER_ACTION_DEFAULT,
            action_timer: PLAYER_ACTION_DURATION_MAP
                .generate_timer(&PLAYER_ACTION_DEFAULT),
            action_duration: PLAYER_ACTION_DURATION_MAP,
            chunk_map: ChunkMap::new(),
            animation: AnimationBundle::new(
                Vec3::new(
                    position.x,
                    position.y + TILE / 2.,
                    PLAYER_SPRITE_LAYER
                ),
                PLAYER_SPRITE_GRID,
                texture,
                texture_atlas
            ),
            animation_frames: PLAYER_SPRITE_INDICES_MAP.clone(),
            offset: TilesetOffset(
                Vec2::new(0., TILE / 2.)
            )
        }
    }
}
