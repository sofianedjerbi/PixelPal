use bevy::prelude::*;
use crate::components::animation::*;


pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(
        &mut AnimationState,
        &mut TextureAtlasSprite,
        &AnimationAction,
        &AnimationPack
    )>,
) {
    for (
        mut animation_state,
        mut sprite,
        animation_action,
        animation_pack
    ) in query.iter_mut() {
        let sprite_animation = animation_pack.get_animation(animation_action);
        animation_state.update(
            &sprite_animation.animation,
            time.delta()
        );
        sprite.index = animation_state.frame_index();
    }
}
