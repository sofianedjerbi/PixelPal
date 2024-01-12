use bevy::prelude::*;

use crate::components::action::*;
use crate::components::animation::*;


/// Updates the sprite animation based on the current action.
///
/// This function iterates through entities with animations and updates their sprite
/// based on the action being performed. It uses the elapsed time to determine the
/// correct frame in the animation sequence.
///
/// # Parameters
/// - `time`: Resource containing time information.
/// - `query`: Query for accessing and modifying animation states, sprites, actions, and frame maps.
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
        state.update(animation, time.delta());
        sprite.index = state.frame_index();
    }
}
