use crate::components::action::*;
use bevy::prelude::*;

/// Bundle for creating a mob in the game.
#[derive(Bundle)]
pub struct ActionBundle {
    current_action: Action,
    action_timer: ActionTimer,
    action_duration: ActionDurationPHF,
}

impl ActionBundle {
    /// Creates a new mob bundle with the specified parameters.
    pub fn new(default_action: &Action, action_duration_map: ActionDurationPHF) -> Self {
        Self {
            current_action: default_action.clone(),
            action_timer: action_duration_map.generate_timer(default_action),
            action_duration: action_duration_map,
        }
    }
}
