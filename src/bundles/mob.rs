use std::sync::atomic::AtomicBool;

use crate::components::action::*;
use crate::components::animation::ActionAnimationMap;
use crate::components::animation::AnimationSpriteGrid;
use crate::components::characters::*;
use crate::components::map::ChunkMap;
use crate::components::textures::TilesetOffset;
use crate::constants::action::*;
use crate::constants::characters::*;
use crate::constants::map::TILE;
use crate::constants::sprites::*;
use bevy::prelude::*;

use super::action::ActionBundle;
use super::animation::ActionAnimationBundle;
use super::animation::AnimationBundle;

/// Bundle for creating a mob in the game.
#[derive(Bundle)]
pub struct MobBundle {
    busy: Busy,
    health: Health,
    action: ActionBundle,
    chunk_map: ChunkMap,
    animation: ActionAnimationBundle,
}

/// Bundle for creating a player mob in the game.
#[derive(Bundle)]
pub struct PlayerMobBundle {
    mob: MobBundle,
}

/// Texture-related parameters
pub struct TextureParameters<'a, 'b, 'c> {
    texture: &'a Handle<Image>,
    texture_atlas: &'b mut ResMut<'c, Assets<TextureAtlas>>,
    sprite_grid: AnimationSpriteGrid,
}

/// Action-related parameters
pub struct ActionParameters<'a> {
    default_action: &'a Action,
    action_duration_map: ActionDurationPHF,
    action_animation_map: &'a ActionAnimationMap,
}

/// Spawn-related parameters
pub struct SpawnParameters<'a> {
    position: &'a Vec2,
    layer: f32,
    health: Health,
}

impl MobBundle {
    /// Creates a new mob bundle with the specified parameters.
    pub fn new(
        spawn_params: SpawnParameters,
        texture_params: TextureParameters,
        action_params: ActionParameters,
    ) -> Self {
        Self {
            busy: Busy(AtomicBool::new(false)),
            health: spawn_params.health,
            action: ActionBundle::new(
                action_params.default_action,
                action_params.action_duration_map,
            ),
            chunk_map: ChunkMap::new(),
            animation: ActionAnimationBundle {
                animation_bundle: AnimationBundle::new(
                    Vec3::new(
                        spawn_params.position.x,
                        spawn_params.position.y + TILE / 2.,
                        spawn_params.layer,
                    ),
                    texture_params.texture_atlas.add(
                        texture_params
                            .sprite_grid
                            .to_atlas(texture_params.texture.clone()),
                    ),
                    TilesetOffset(Vec2::new(0., TILE / 2.)),
                ),
                action_animation_map: action_params.action_animation_map.clone(),
            },
        }
    }
}

impl PlayerMobBundle {
    /// Creates a new player mob bundle with the specified parameters.
    pub fn new(
        position: Vec2,
        texture: &Handle<Image>,
        texture_atlas: &mut ResMut<Assets<TextureAtlas>>,
    ) -> Self {
        let spawn_params = SpawnParameters {
            position: &position,
            layer: PLAYER_SPRITE_LAYER,
            health: PLAYER_HEALTH,
        };

        let texture_params = TextureParameters {
            texture,
            texture_atlas,
            sprite_grid: PLAYER_SPRITE_GRID,
        };

        let action_params = ActionParameters {
            default_action: &PLAYER_ACTION_DEFAULT,
            action_duration_map: PLAYER_ACTION_DURATION_MAP,
            action_animation_map: &PLAYER_SPRITE_INDICES_MAP,
        };

        Self {
            mob: MobBundle::new(spawn_params, texture_params, action_params),
        }
    }
}
