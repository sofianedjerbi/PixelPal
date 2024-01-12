use bevy::{prelude::*, utils::HashMap};
use once_cell::sync::Lazy;

use crate::components::action::*;
use crate::components::animation::*;
use crate::util::animation::new_animation;

// PLAYER
pub const PLAYER_SPRITE: &str = "characters/player.png";
pub const PLAYER_SPRITE_LAYER: f32 = 4.;

pub const PLAYER_SPRITE_GRID: AnimationSpriteGrid = AnimationSpriteGrid {
    size: Vec2::new(48., 48.),
    columns: 8,
    rows: 24,
    padding: None,
    offset: None,
};

pub const PLAYER_STAND_FPS: f64 = 5.;
pub const PLAYER_WALK_FPS: f64 = 15.;
pub const PLAYER_RUN_FPS: f64 = 15.;

pub static PLAYER_SPRITE_INDICES_MAP: Lazy<ActionAnimationMap> = Lazy::new(|| {
    ActionAnimationMap(HashMap::from([
        // Stand Actions
        (
            Action::new(ActionKind::Stand, ActionDirection::Down),
            new_animation(0..8, PLAYER_STAND_FPS),
        ),
        (
            Action::new(ActionKind::Stand, ActionDirection::Up),
            new_animation(8..16, PLAYER_STAND_FPS),
        ),
        (
            Action::new(ActionKind::Stand, ActionDirection::Left),
            new_animation(16..24, PLAYER_STAND_FPS),
        ),
        (
            Action::new(ActionKind::Stand, ActionDirection::Right),
            new_animation(24..32, PLAYER_STAND_FPS),
        ),
        // Walk Actions
        (
            Action::new(ActionKind::Walk, ActionDirection::Down),
            new_animation(32..40, PLAYER_WALK_FPS),
        ),
        (
            Action::new(ActionKind::Walk, ActionDirection::Up),
            new_animation(40..48, PLAYER_WALK_FPS),
        ),
        (
            Action::new(ActionKind::Walk, ActionDirection::Right),
            new_animation(48..56, PLAYER_WALK_FPS),
        ),
        (
            Action::new(ActionKind::Walk, ActionDirection::Left),
            new_animation(56..64, PLAYER_WALK_FPS),
        ),
        // Sprint Actions
        (
            Action::new(ActionKind::Run, ActionDirection::Down),
            new_animation(64..72, PLAYER_RUN_FPS),
        ),
        (
            Action::new(ActionKind::Run, ActionDirection::Up),
            new_animation(72..80, PLAYER_RUN_FPS),
        ),
        (
            Action::new(ActionKind::Run, ActionDirection::Right),
            new_animation(80..88, PLAYER_RUN_FPS),
        ),
        (
            Action::new(ActionKind::Run, ActionDirection::Left),
            new_animation(88..96, PLAYER_RUN_FPS),
        ),
    ]))
});
