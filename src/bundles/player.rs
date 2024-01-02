use bevy::prelude::*;
use crate::components::action::*;
use crate::components::animation::*;
use crate::components::characters::*;
use crate::components::map::ChunkMap;
use crate::constants::action::PLAYER_ACTION_DEFAULT;
use crate::constants::action::PLAYER_ACTION_DURATION_MAP;
use crate::constants::characters::*;
use crate::constants::sprites::*;


#[derive(Bundle)]
pub struct PlayerBundle {
    pub busy: Busy,
    pub health: Health,
    pub sprite: SpriteSheetBundle,
    pub current_action: Action,
    pub animation_state: AnimationState,
    pub action_timer: ActionTimer,
    pub animation_frames: AnimationFramesMap,
    pub action_duration: ActionDurationPHF,
    pub chunk_map: ChunkMap
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
                    position.y + PLAYER_SPRITE_SIZE.y / 2.,
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
            current_action: PLAYER_ACTION_DEFAULT,
            animation_state: AnimationState::default(),
            action_timer: PLAYER_ACTION_DURATION_MAP
                .generate_timer(&PLAYER_ACTION_DEFAULT),
            animation_frames: PLAYER_SPRITE_INDICES_MAP.clone(),
            action_duration: PLAYER_ACTION_DURATION_MAP,
            chunk_map: ChunkMap::new()
        }
    }
}
