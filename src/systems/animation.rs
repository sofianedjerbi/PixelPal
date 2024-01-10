use bevy::prelude::*;

use crate::components::action::*;
use crate::components::animation::*;


pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(
        &mut AnimationState,
        &mut TextureAtlasSprite,
        &Action,
        &ActionAnimationMap
    )>,
) {
    for (
        mut state,
        mut sprite,
        action,
        frames
    ) in query.iter_mut() {
        let animation = frames.lookup(action);
        state.update(&animation, time.delta());
        sprite.index = state.frame_index();
    }
}
