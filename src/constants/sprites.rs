use bevy::{prelude::*, utils::HashMap};
use once_cell::sync::Lazy;

use crate::components::action::*;
use crate::components::animation::*;


// PLAYER
pub const PLAYER_SPRITE: &str = "characters/player.png";
pub const PLAYER_SPRITE_SIZE: Vec2 = Vec2::new(48., 48.);
pub const PLAYER_SPRITE_OFFSET: Option<Vec2> = None;
pub const PLAYER_SPRITE_PADDING: Option<Vec2> = None;
pub const PLAYER_SPRITE_COLUMNS: usize = 8;
pub const PLAYER_SPRITE_ROWS: usize = 24;
pub const PLAYER_SPRITE_LAYER: f32 = 4.;

pub const PLAYER_STAND_FPS: f64 = 5.;
pub const PLAYER_WALK_FPS: f64 = 15.;
pub const PLAYER_RUN_FPS: f64 = 15.;

pub static PLAYER_SPRITE_INDICES_MAP: Lazy<AnimationFramesMap> = Lazy::new(|| {
    AnimationFramesMap(
        HashMap::from([
            // Stand Actions
            (
                Action::new(ActionKind::Stand, ActionDirection::Down),
                SpriteAnimation::new(0..8, PLAYER_STAND_FPS)
            ),
            (
                Action::new(ActionKind::Stand, ActionDirection::Up),
                SpriteAnimation::new(8..16, PLAYER_STAND_FPS)
            ),
            (
                Action::new(ActionKind::Stand, ActionDirection::Left),
                SpriteAnimation::new(16..24, PLAYER_STAND_FPS)
            ),
            (
                Action::new(ActionKind::Stand, ActionDirection::Right),
                SpriteAnimation::new(24..32, PLAYER_STAND_FPS)
            ),
            // Walk Actions
            (
                Action::new(ActionKind::Walk, ActionDirection::Down),
                SpriteAnimation::new(32..40, PLAYER_WALK_FPS)
            ),
            (
                Action::new(ActionKind::Walk, ActionDirection::Up),
                SpriteAnimation::new(40..48, PLAYER_WALK_FPS)
            ),
            (
                Action::new(ActionKind::Walk, ActionDirection::Right),
                SpriteAnimation::new(48..56, PLAYER_WALK_FPS)
            ),
            (
                Action::new(ActionKind::Walk, ActionDirection::Left),
                SpriteAnimation::new(56..64, PLAYER_WALK_FPS)
            ),
            // Sprint Actions
            (
                Action::new(ActionKind::Run, ActionDirection::Down),
                SpriteAnimation::new(64..72, PLAYER_RUN_FPS)
            ),
            (
                Action::new(ActionKind::Run, ActionDirection::Up),
                SpriteAnimation::new(72..80, PLAYER_RUN_FPS)
            ),
            (
                Action::new(ActionKind::Run, ActionDirection::Right),
                SpriteAnimation::new(80..88, PLAYER_RUN_FPS)
            ),
            (
                Action::new(ActionKind::Run, ActionDirection::Left),
                SpriteAnimation::new(88..96, PLAYER_RUN_FPS)
            ),
        ])
    )
});
