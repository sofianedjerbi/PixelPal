use bevy::prelude::*;
use crate::components::animation::{SpriteAnimation, SpriteAnimationState};


pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(
        &mut SpriteAnimationState,
        &mut TextureAtlasSprite,
        &SpriteAnimation
    )>,
) {
    for (mut animation_state, mut sprite, animation) in query.iter_mut() {
        animation_state.update(
            animation,
            time.delta()
        );
        sprite.index = animation_state.frame_index();
    }
}
