use bevy::prelude::*;
use once_cell::sync::Lazy;

use crate::components::action::*;
use crate::components::characters::*;

use super::mapping::TILE;


// ALL CHARACTERS
#[allow(non_snake_case)]
pub const fn ACTION_TRANSFORMATION(
    action: &Action
) -> Vec3 {
    let norm = match action.action_type {
        ActionType::Walking => 1 * TILE as i32,
        _ => 0 * TILE as i32,
    };

    let vector = match action.direction {
        ActionDirection::Up => Vec2::new(0., norm as f32),
        ActionDirection::Down => Vec2::new(0., -norm as f32),
        ActionDirection::Left => Vec2::new(-norm as f32, 0.),
        ActionDirection::Right => Vec2::new(norm as f32, 0.),
    };

    Vec3::new(vector.x, vector.y, 0.)
}


// PLAYER
#[allow(non_snake_case)]
pub fn PLAYER_ACTION_TIMER(
    action_type: &ActionType
) -> Timer {
    match action_type {
        ActionType::Walking => 
            Timer::from_seconds(0.3, TimerMode::Once),
        _ => 
            Timer::default(),
    }
}

pub const PLAYER_HEALTH: Health = Health(100);
pub const PLAYER_ANIMATION_DEFAULT: Lazy<Action> = Lazy::new(|| {
    Action::new_once(
        ActionType::Standing,
        ActionDirection::Down,
        0.
    )
});

// User
pub const USER_SPAWN: Vec2 = Vec2::new(TILE * 0., TILE * 0.);
// Mittens
pub const MITTENS_SPAWN: Vec2 = Vec2::new(TILE * 4., TILE * 0.);



