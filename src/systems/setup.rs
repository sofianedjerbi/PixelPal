use std::env;

use crate::bundles::gpt::GptBundle;
use crate::bundles::player::PlayerBundle;
use crate::components::characters::*;
use crate::components::display::IsGameCamera;
use crate::components::map::MainTilemapTexture;
use crate::constants::characters::*;
use crate::constants::display::*;
use crate::constants::sprites::PLAYER_SPRITE;
use crate::constants::textures::TEXTURE_PATH;
use bevy::prelude::*;
use bevy_pixel_camera::*;

/// Sets up the initial game environment.
///
/// This system initializes the game world by loading textures,
/// spawning the main camera, and creating player and bot entities.
///
/// # Parameters
/// - `commands`: Commands for spawning entities and resources.
/// - `asset_server`: Resource to load assets.
/// - `main_texture`: Mutable resource for the main tilemap texture.
/// - `textures`: Mutable resource for managing texture atlases.
pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut main_texture: ResMut<MainTilemapTexture>,
    mut textures: ResMut<Assets<TextureAtlas>>,
) {
    // Load tileset
    main_texture.set_handle(asset_server.load(TEXTURE_PATH));

    // Spawn the camera
    commands
        .spawn((
            Camera2dBundle {
                //transform: Transform::from_scale(Vec3::new(5., 5., 1.)),
                ..Default::default()
            },
            PixelZoom::Fixed(ZOOM),
            PixelViewport,
        ))
        .insert(IsGameCamera);

    // Spawn Player
    let player_texture = asset_server.load(PLAYER_SPRITE);

    commands
        .spawn(PlayerBundle::new(
            USER_SPAWN,
            player_texture.clone(),
            &mut textures,
        ))
        .insert(IsUser);

    // Spawn Mittens (GPT)
    let option_key = env::var("GPT_KEY");
    if let Ok(key) = option_key {
        let option_gpt = GptBundle::new(MITTENS_SPAWN, player_texture.clone(), &mut textures, &key);
        if let Some(gpt) = option_gpt {
            commands.spawn(gpt).insert(IsBot);
        }
    }
}
