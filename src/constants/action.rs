use std::time::Duration;
use phf::phf_map;
use crate::components::action::*;
use super::map::TILE;

pub const ACTION_TICK_FREQUENCY: Duration = Duration::from_millis(20);
pub const WALK_RATE: f32 = 1. * TILE;
pub const RUN_RATE: f32 = 1. * TILE;


// PLAYER VALUES
pub const PLAYER_ACTION_DEFAULT: Action = Action::new(
    ActionKind::Stand,
    ActionDirection::Down,
);

pub const PLAYER_ACTION_DURATION_MAP: ActionDurationPHF = ActionDurationPHF(
    phf_map! {
        "Stand" => 0.0,
        "Walk" => 0.3,
        "Run" => 0.2,
    }
);
