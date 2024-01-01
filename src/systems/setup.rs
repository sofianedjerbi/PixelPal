use std::env;

use bevy::prelude::*;
use bevy_pixel_camera::*;
use crate::bundles::gpt::GptBundle;
use crate::bundles::player::PlayerBundle;
use crate::components::flags::*;
use crate::constants::characters::*;
use crate::constants::display::*;

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut textures: ResMut<Assets<TextureAtlas>>
 ) {
    // Spawn the camera
    commands.spawn((
        Camera2dBundle {
            ..Default::default()
        },
        PixelZoom::Fixed(ZOOM),
        PixelViewport,
    )).insert(IsGameCamera);

    // Spawn Player
    commands.spawn(
        PlayerBundle::new(USER_SPAWN, &asset_server, &mut textures)
    ).insert(IsUser);

    // Spawn Mittens (GPT)
    let option_key = env::var("GPT_KEY");
    if let Ok(key) = option_key {
        let option_gpt = GptBundle::new(MITTENS_SPAWN, &asset_server, &mut textures, &key);
        if let Some(gpt) = option_gpt {
            commands.spawn(gpt).insert(IsBot);
        }
    }
}
