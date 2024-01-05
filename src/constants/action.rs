use std::time::Duration;
use phf::phf_map;
use crate::components::action::*;
use super::map::TILE;

pub const ACTION_TICK_FREQUENCY: Duration = Duration::from_millis(20);
pub const WALKING_BPS: f32 = 1. * TILE;

// PLAYER VALUES
pub const PLAYER_ACTION_DEFAULT: Action = Action::new(
    ActionKind::Standing,
    ActionDirection::Down
);

pub const PLAYER_ACTION_DURATION_MAP: ActionDurationPHF = ActionDurationPHF(
    phf_map! {
        "Standing" => 0.0,
        "Walking" => 0.3,
    }
);
