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
        sprite_grid: AnimationSpriteGrid,
        texture: Handle<Image>,
        texture_atlas: &mut ResMut<Assets<TextureAtlas>>,
    ) -> Self {
        Self {
            sprite: SpriteSheetBundle {
                transform: Transform::from_xyz(
                    position.x,
                    position.y,
                    position.z
                ),
                texture_atlas: texture_atlas.add(
                    sprite_grid.to_atlas(texture)
                ),
                ..Default::default()
            },
            animation_state: AnimationState::default(),
        }
    }
}

