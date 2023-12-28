use bevy::prelude::*;
use crate::components::characters::*;
use crate::constants::mapping::TILE_SIZE;


// SPRITE CONFIG
const SPRITE: &str = "characters/player.png";
const SIZE: Vec2 = Vec2::new(16., 16.);
const OFFSET: Option<Vec2> = Some(Vec2::new(16., 16.));
const PADDING: Option<Vec2> = Some(Vec2::new(32., 32.));
const COLUMNS: usize = 4;
const ROWS: usize = 4;

// BUNDLE DEFAULT
const HEALTH: u8 = 100;

// OTHER
const LAYER: f32 = 4.;


#[derive(Bundle)]
pub struct PlayerBundle {
    pub busy: Busy,
    pub health: Health,
    pub sprite: SpriteSheetBundle
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
                transform: Transform { 
                    translation: Vec3::new(0., 0., LAYER),
                    ..Default::default()
                },
                texture_atlas: textures.add(
                        TextureAtlas::from_grid(
                        asset_server.load(SPRITE),
                        TILE_SIZE,
                        COLUMNS,
                        ROWS,
                        PADDING,
                        OFFSET
                    )
                ),
                ..Default::default() // fill in the rest of the fields with default values
            },
        }
    }
}
