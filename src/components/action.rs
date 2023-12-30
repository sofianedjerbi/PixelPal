use bevy::prelude::*;
use phf::Map;
use strum_macros::Display;

use crate::constants::mapping::TILE;
use crate::constants::action::WALKING_BPS;


#[derive(Component, Clone, PartialEq, Eq, Hash, Display)]
pub enum ActionDirection {
    Up,
    Down,
    Left,
    Right
}

#[derive(Component, Clone, PartialEq, Eq, Hash, Display)]
pub enum ActionKind {
    Standing,
    Walking,
    // Add future actions here
}

#[derive(Component, Clone, PartialEq, Eq, Hash)]
pub struct Action {
    pub kind: ActionKind,
    pub direction: ActionDirection
}

impl Action {
    pub const fn new(
        kind: ActionKind,
        direction: ActionDirection
    ) -> Self {
        Self { kind, direction }
    }

    pub const fn get_transformation(&self) -> Vec3 {
        let norm = match self.kind {
            ActionKind::Walking => WALKING_BPS as i32,
            _ => 0 * TILE as i32,
        };

        let vector = match self.direction {
            ActionDirection::Up => Vec2::new(0., norm as f32),
            ActionDirection::Down => Vec2::new(0., -norm as f32),
            ActionDirection::Left => Vec2::new(-norm as f32, 0.),
            ActionDirection::Right => Vec2::new(norm as f32, 0.),
        };

        Vec3::new(vector.x, vector.y, 0.)
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct ActionTimer(pub Timer);

#[derive(Component)]
pub struct ActionDurationPHF(
    pub Map<&'static str, f32>
);

impl ActionDurationPHF {
    pub fn lookup(&self, action: &Action) -> f32 {
        self.0.get(&action.kind.to_string())
              .unwrap() // We're unwrapping hardcoded values.
              .clone()
    }

    pub fn generate_timer(&self, action: &Action) -> ActionTimer {
        ActionTimer(Timer::from_seconds(
            self.lookup(action),
            TimerMode::Once
        ))
    }
}
