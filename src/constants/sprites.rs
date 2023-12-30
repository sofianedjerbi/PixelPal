use bevy::prelude::*;
use once_cell::sync::Lazy;

use crate::components::animation::*;


// PLAYER
pub const PLAYER_SPRITE: &str = "characters/player.png";
pub const PLAYER_SPRITE_SIZE: Vec2 = Vec2::new(48., 48.);
pub const PLAYER_SPRITE_OFFSET: Option<Vec2> = None;
pub const PLAYER_SPRITE_PADDING: Option<Vec2> = None;
pub const PLAYER_SPRITE_COLUMNS: usize = 8;
pub const PLAYER_SPRITE_ROWS: usize = 24;
pub const PLAYER_SPRITE_LAYER: f32 = 4.;
pub static PLAYER_ANIMATIONS: Lazy<AnimationPack> = Lazy::new(|| { AnimationPack {
        standing: AnimationMovement {
            down: SpriteAnimation::new(0..8, 5.),
            up: SpriteAnimation::new(8..16, 5.),
            left: SpriteAnimation::new(16..24, 5.),
            right: SpriteAnimation::new(24..32, 5.),
        },
        walking: AnimationMovement {
            down: SpriteAnimation::new(32..40, 12.),
            up: SpriteAnimation::new(40..48, 12.),
            right: SpriteAnimation::new(48..56, 12.),
            left: SpriteAnimation::new(56..64, 12.),
        },
        // ... initialize other animations if any ...
    }
});
