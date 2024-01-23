use bevy::prelude::*;

use crate::components::{
    animation::{AnimationSpriteGrid, DefinedAnimation},
    texture::TilesetOffset,
};

use super::animation::{AnimationBundle, SingleAnimationBundle};

#[derive(Bundle)]
pub struct EffectBundle {
    animation: SingleAnimationBundle,
}

impl EffectBundle {
    /// Creates a new effect.
    pub fn new(
        animation: DefinedAnimation,
        sprite_grid: &AnimationSpriteGrid,
        position: Vec3,
        texture: &Handle<Image>,
        texture_atlas: &mut ResMut<Assets<TextureAtlas>>,
    ) -> Self {
        Self {
            animation: SingleAnimationBundle {
                animation_bundle: AnimationBundle::new(
                    Vec3::new(position.x, position.y, position.z),
                    texture_atlas.add(sprite_grid.to_atlas(texture.clone())),
                    TilesetOffset(Vec2::new(0., 0.)),
                ),
                animation,
            },
        }
    }
}
