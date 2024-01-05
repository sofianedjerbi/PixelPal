use bevy::prelude::*;
use phf::Map;
use strum_macros::{Display, EnumString};
use schemars::JsonSchema;
use serde::Deserialize;
use std::str::FromStr;

use crate::constants::map::TILE;
use crate::constants::action::*;


#[derive(Component, Debug, Clone, PartialEq, Eq, Hash, Display, JsonSchema, Deserialize, EnumString)]
pub enum ActionDirection {
    #[strum(ascii_case_insensitive)]
    Up,
    #[strum(ascii_case_insensitive)]
    Down,
    #[strum(ascii_case_insensitive)]
    Left,
    #[strum(ascii_case_insensitive)]
    Right
}

#[derive(Component, Debug, Clone, PartialEq, Eq, Hash, Display, JsonSchema, Deserialize, EnumString)]
pub enum ActionKind {
    #[strum(ascii_case_insensitive)]
    Stand,
    #[strum(ascii_case_insensitive)]
    Walk,
    #[strum(ascii_case_insensitive)]
    Run,
    // Add future actions here
}

#[derive(Component, Debug, Clone, PartialEq, Eq, Hash, JsonSchema, Deserialize)]
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
            ActionKind::Walk => WALK_RATE as i32,
            ActionKind::Run => RUN_RATE as i32,
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

    pub fn from_command_string(commands: &str) -> Option<Vec<Action>> {
        let uppercase = commands.to_uppercase();
        let mut actions = Vec::new();
        for line in uppercase.lines() {
            if let Some(mut line_actions) = Self::from_single_command_string(line) {
                actions.append(&mut line_actions);
            }
        }
        if actions.is_empty() {
            None
        } else {
            Some(actions)
        }
    }

    fn from_single_command_string(command: &str) -> Option<Vec<Action>> {
        let parts: Vec<&str> = command.split_whitespace().collect();
        if parts.len() < 2 || parts.len() > 3 {
            return None;
        }

        let mut kind = ActionKind::from_str(parts[0]).ok()?;
        let direction = ActionDirection::from_str(parts[1]).ok()?;

        let times = if parts.len() == 3 {
            parts[2].parse::<usize>().ok()? 
        } else {
            1
        };

        if times > 5 && kind == ActionKind::Walk {
            kind = ActionKind::Run
        }

        Some(vec![Action { kind, direction }; times])
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
