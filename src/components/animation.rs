use benimator::*;
use bevy::prelude::*;

use super::action::*;


/// Holds the current state of an animation.
#[derive(Default, Component, Deref, DerefMut)]
pub struct AnimationState(pub benimator::State);

/// Contains an animation, such as walking or jumping.
#[derive(Component, Deref, Clone)]
pub struct SpriteAnimation(Animation);

impl SpriteAnimation {
    /// Construct with indices & fps
    pub fn new(
        indices: impl IntoIterator<Item = usize>,
        fps: f64
    ) -> Self {
        Self(Animation::from_indices(indices, FrameRate::from_fps(fps)))
    }
}

#[derive(Component, Clone)]
pub struct AnimationMovement {
    pub up: SpriteAnimation,
    pub down: SpriteAnimation,
    pub left: SpriteAnimation,
    pub right: SpriteAnimation
}

#[derive(Component, Clone)]
pub struct AnimationPack {
    pub standing: AnimationMovement,
    pub walking: AnimationMovement,
    // Other animations...
}

impl AnimationPack {
    pub fn get_animation(&self, action: &Action) -> &SpriteAnimation {
        let movement = match action.action_type {
            ActionType::Standing => &self.standing,
            ActionType::Walking => &self.walking,
            // ... handle other actions if necessary
        };

        match action.direction {
            ActionDirection::Up => &movement.up,
            ActionDirection::Down => &movement.down,
            ActionDirection::Left => &movement.left,
            ActionDirection::Right => &movement.right,
        }
    }
}
