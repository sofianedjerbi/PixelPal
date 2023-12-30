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

pub const PLAYER_STANDING_FPS: f64 = 5.;
pub const PLAYER_WALKING_FPS: f64 = 16.;

pub static PLAYER_SPRITE_INDICES_MAP: Lazy<AnimationFramesMap> = Lazy::new(|| {
    AnimationFramesMap(
        HashMap::from([
            // Standing Actions
            (
                Action::new(ActionKind::Standing, ActionDirection::Down),
                SpriteAnimation::new(0..8, PLAYER_STANDING_FPS)
            ),
            (
                Action::new(ActionKind::Standing, ActionDirection::Up),
                SpriteAnimation::new(8..16, PLAYER_STANDING_FPS)
            ),
            (
                Action::new(ActionKind::Standing, ActionDirection::Left),
                SpriteAnimation::new(16..24, PLAYER_STANDING_FPS)
            ),
            (
                Action::new(ActionKind::Standing, ActionDirection::Right),
                SpriteAnimation::new(24..32, PLAYER_STANDING_FPS)
            ),
            // Walking Actions
            (
                Action::new(ActionKind::Walking, ActionDirection::Down),
                SpriteAnimation::new(32..40, PLAYER_WALKING_FPS)
            ),
            (
                Action::new(ActionKind::Walking, ActionDirection::Up),
                SpriteAnimation::new(40..48, PLAYER_WALKING_FPS)
            ),
            (
                Action::new(ActionKind::Walking, ActionDirection::Right),
                SpriteAnimation::new(48..56, PLAYER_WALKING_FPS)
            ),
            (
                Action::new(ActionKind::Walking, ActionDirection::Left),
                SpriteAnimation::new(56..64, PLAYER_WALKING_FPS)
            ),
        ])
    )
});
