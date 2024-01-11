use bevy::prelude::*;

use crate::components::animation::*;


#[derive(Bundle)]
pub struct AnimationBundle {
    pub sprite: SpriteSheetBundle,
    pub animation_state: AnimationState,
}

impl AnimationBundle {
    pub fn new(
        position: Vec3,
        texture_atlas: Handle<TextureAtlas>,
    ) -> Self {
        Self {
            sprite: SpriteSheetBundle {
                transform: Transform::from_xyz(
                    position.x,
                    position.y,
                    position.z
                ),
                texture_atlas,
                ..Default::default()
            },
            animation_state: AnimationState::default(),
        }
    }
}

#[derive(Bundle)]
pub struct ActionAnimationBundle {
    pub animation_bundle: AnimationBundle,
    pub action_animation_map: ActionAnimationMap,
}
