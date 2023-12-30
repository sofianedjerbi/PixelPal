use bevy::prelude::*;
use crate::components::action::*;
use crate::components::animation::*;
use crate::components::characters::*;
use crate::constants::characters::*;
use crate::constants::sprites::*;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub busy: Busy,
    pub health: Health,
    pub sprite: SpriteSheetBundle,
    pub animation_pack: AnimationPack,
    pub current_action: Action,
    pub animation_state: AnimationState,
}

impl PlayerBundle {
    pub fn new(
        position: Vec2,
        asset_server: &Res<AssetServer>,
        textures: &mut ResMut<Assets<TextureAtlas>>
    ) -> Self {
        PlayerBundle {
            busy: Busy(false),
            health: PLAYER_HEALTH,
            sprite: SpriteSheetBundle {
                transform: Transform::from_xyz(
                    position.x,
                    position.y + PLAYER_SPRITE_SIZE.x / 2.,
                    PLAYER_SPRITE_LAYER),
                texture_atlas: textures.add(
                        TextureAtlas::from_grid(
                        asset_server.load(PLAYER_SPRITE),
                        PLAYER_SPRITE_SIZE.into(),
                        PLAYER_SPRITE_COLUMNS,
                        PLAYER_SPRITE_ROWS,
                        PLAYER_SPRITE_PADDING,
                        PLAYER_SPRITE_OFFSET
                    )
                ),
                ..Default::default()
            },
            animation_pack: PLAYER_ANIMATIONS.clone(),
            current_action: PLAYER_ANIMATION_DEFAULT.clone(),
            animation_state: AnimationState::default(),
        }
    }
}
