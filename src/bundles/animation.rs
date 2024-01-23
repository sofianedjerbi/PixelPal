use bevy::prelude::*;

use crate::components::{animation::*, texture::TilesetOffset};

/// Bundle for animated entities.
#[derive(Bundle)]
pub struct AnimationBundle {
    sprite: SpriteSheetBundle,
    animation_state: AnimationState,
    offset: TilesetOffset,
}

/// Bundle for animated entities that use actions.
#[derive(Bundle)]
pub struct ActionAnimationBundle {
    pub animation_bundle: AnimationBundle,
    pub action_animation_map: ActionAnimationMap,
}

/// Bundle for a single animation.
#[derive(Bundle)]
pub struct SingleAnimationBundle {
    pub animation_bundle: AnimationBundle,
    pub animation: DefinedAnimation,
}

impl AnimationBundle {
    pub fn new(position: Vec3, texture_atlas: Handle<TextureAtlas>, offset: TilesetOffset) -> Self {
        Self {
            sprite: SpriteSheetBundle {
                transform: Transform::from_xyz(position.x, position.y, position.z),
                texture_atlas,
                ..Default::default()
            },
            animation_state: AnimationState::default(),
            offset,
        }
    }
}
