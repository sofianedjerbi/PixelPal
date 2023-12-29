use bevy::prelude::*;
use crate::components::animation::*;
use crate::components::characters::*;
use crate::constants::sprites::*;


// BUNDLE DEFAULT
const HEALTH: u8 = 100;


#[derive(Bundle)]
pub struct PlayerBundle {
    pub busy: Busy,
    pub health: Health,
    pub sprite: SpriteSheetBundle,
    pub animation_pack: AnimationPack,
    pub current_animation: AnimationAction,
    pub animation_state: AnimationState,
}

impl PlayerBundle {
    pub fn new(
        asset_server: Res<AssetServer>,
        mut textures: ResMut<Assets<TextureAtlas>>
    ) -> Self {
        PlayerBundle {
            busy: Busy(false),
            health: Health(HEALTH),
            sprite: SpriteSheetBundle {
                transform: Transform::from_xyz(0., 4., PLAYER_SPRITE_LAYER),
                texture_atlas: textures.add(
                        TextureAtlas::from_grid(
                        asset_server.load(PLAYER_SPRITE),
                        TILE_SIZE.into(),
                        PLAYER_SPRITE_COLUMNS,
                        PLAYER_SPRITE_ROWS,
                        PLAYER_SPRITE_PADDING,
                        PLAYER_SPRITE_OFFSET
                    )
                ),
                ..Default::default()
            },
            animation_pack: PLAYER_ANIMATIONS.clone(),
            current_animation: PLAYER_ANIMATION_DEFAULT,
            animation_state: AnimationState::default()
        }
    }
}
