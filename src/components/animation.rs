use benimator::*;
use bevy::{prelude::*, utils::HashMap};

use super::action::Action;


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


#[derive(Component, Clone, Deref)]
pub struct AnimationFramesMap(
    pub HashMap<Action, SpriteAnimation>
);

impl AnimationFramesMap {
    pub fn lookup(&self, action: &Action) -> &SpriteAnimation {
        // We're unwrapping hardcoded values.
        self.0.get(action).unwrap()
    }
}

#[derive(Component)]
pub struct IsGameCamera;

#[derive(Component, Deref)]
pub struct TileAnimation(Option<SpriteAnimation>);
