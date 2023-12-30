use bevy::prelude::*;


#[derive(Component, Clone, Debug, PartialEq, Eq)]
pub enum ActionDirection {
    Up,
    Down,
    Left,
    Right
}

#[derive(Component, Clone, Debug, PartialEq, Eq)]
pub enum ActionType {
    Standing,
    Walking,
    // Add future actions here
}

#[derive(Component, Clone, Debug, PartialEq, Eq)]
pub struct Action {
    pub action_type: ActionType,
    pub direction: ActionDirection,
    pub timer: Timer
}

impl Action {
    pub fn new_once(
        action_type: ActionType,
        direction: ActionDirection,
        duration: f32,
    ) -> Self {
        Self {
            action_type,
            direction,
            timer: Timer::from_seconds(duration, TimerMode::Once)
        }
    }
}
