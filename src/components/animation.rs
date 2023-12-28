use benimator::*;
use bevy::prelude::{Component, DerefMut, Deref};


/// Holds the current state of an animation.
#[derive(Default, Component, Deref, DerefMut)]
pub struct SpriteAnimationState(pub State);

/// Contains an animation, such as walking or jumping.
#[derive(Component, Deref)]
pub struct SpriteAnimation(pub Animation);

impl SpriteAnimation {
    /// Construct with indices & fps
    pub fn new(indices: impl IntoIterator<Item = usize>, fps: f64) -> Self {
        let animation = Animation::from_indices(indices, FrameRate::from_fps(fps));
        SpriteAnimation(animation)
    }
}
