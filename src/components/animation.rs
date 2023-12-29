use benimator::*;
use bevy::prelude::*;


/// Holds the current state of an animation.
#[derive(Default, Component, Deref, DerefMut)]
pub struct AnimationState(pub benimator::State);

/// Contains an animation, such as walking or jumping.
#[derive(Component, Deref, Clone)]
pub struct SpriteAnimation {
    pub animation: Animation
}

impl SpriteAnimation {
    /// Construct with indices & fps
    pub fn new(
        indices: impl IntoIterator<Item = usize>,
        fps: f64
    ) -> Self {
        Self { 
            animation: Animation::from_indices(indices, FrameRate::from_fps(fps))
        }
    }
}

#[derive(Component, Clone, Debug)]
pub enum AnimationDirection {
    Up,
    Down,
    Left,
    Right
}

#[derive(Component, Clone, Debug)]
pub enum ActionType {
    Standing,
    Walking,
    // Add future actions here
}

#[derive(Component, Clone, Debug)]
pub struct AnimationAction {
    pub action_type: ActionType,
    pub direction: AnimationDirection
}

impl AnimationAction {
    pub const fn new(
        action_type: ActionType,
        direction: AnimationDirection,
    ) -> Self {
        Self {
            action_type,
            direction
        }
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
    pub fn get_animation(&self, action: &AnimationAction) -> &SpriteAnimation {
        let movement = match action.action_type {
            ActionType::Standing => &self.standing,
            ActionType::Walking => &self.walking,
            // ... handle other actions if necessary
        };

        match action.direction {
            AnimationDirection::Up => &movement.up,
            AnimationDirection::Down => &movement.down,
            AnimationDirection::Left => &movement.left,
            AnimationDirection::Right => &movement.right,
        }
    }
}
