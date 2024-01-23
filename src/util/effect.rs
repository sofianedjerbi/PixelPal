use bevy::prelude::*;

use crate::{
    bundles::effect::EffectBundle,
    components::animation::{AnimationSpriteGrid, DefinedAnimation},
};

pub fn spawn_effect(
    commands: &mut Commands,
    sprite_grid: &AnimationSpriteGrid,
    animation: DefinedAnimation,
    position: Vec3,
    texture: &Handle<Image>,
    texture_atlas: &mut ResMut<Assets<TextureAtlas>>,
) {
    commands.spawn(EffectBundle::new(
        animation,
        sprite_grid,
        position,
        texture,
        texture_atlas,
    ));
}
