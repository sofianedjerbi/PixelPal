use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use once_cell::sync::Lazy;

use crate::components::animation::*;


pub const TILE_SIZE: TilemapTileSize = TilemapTileSize { x: 16.0, y: 16.0 };

// PLAYER
pub const PLAYER_SPRITE: &str = "characters/player.png";
pub const PLAYER_SPRITE_OFFSET: Option<Vec2> = Some(Vec2::new(16., 16.));
pub const PLAYER_SPRITE_PADDING: Option<Vec2> = Some(Vec2::new(32., 32.));
pub const PLAYER_SPRITE_COLUMNS: usize = 4;
pub const PLAYER_SPRITE_ROWS: usize = 4;
pub const PLAYER_SPRITE_LAYER: f32 = 4.;
pub const PLAYER_ANIMATION_DEFAULT: AnimationAction = AnimationAction::new(
    ActionType::Standing,
    AnimationDirection::Down
);
pub static PLAYER_ANIMATIONS: Lazy<AnimationPack> = Lazy::new(|| { AnimationPack {
        standing: AnimationMovement {
            down: SpriteAnimation::new([0, 1], 0.7),
            up: SpriteAnimation::new([4, 5], 0.7),
            left: SpriteAnimation::new([8, 9], 0.7),
            right: SpriteAnimation::new([12, 13], 0.7),
        },
        walking: AnimationMovement {
            down: SpriteAnimation::new([2, 3], 2.),
            up: SpriteAnimation::new([6, 7], 2.),
            left: SpriteAnimation::new([10, 11], 2.),
            right: SpriteAnimation::new([14, 15], 2.),
        },
        // ... initialize other animations if any ...
    }
});
