use std::env;

use bevy::prelude::*;
use bevy_pixel_camera::*;
use crate::bundles::gpt::GptBundle;
use crate::bundles::player::PlayerBundle;
use crate::components::animation::IsGameCamera;
use crate::components::characters::*;
use crate::components::map::MainTilemapTexture;
use crate::constants::characters::*;
use crate::constants::display::*;

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut main_texture: ResMut<MainTilemapTexture>,
    mut textures: ResMut<Assets<TextureAtlas>>
 ) {
    // Load tileset
    main_texture.set_handle(
        asset_server.load("tileset/environment/full.png")
    );

    // Spawn the camera
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_scale(Vec3::new(5., 5., 1.)),
            ..Default::default()
        },
        //PixelZoom::Fixed(ZOOM),
        //PixelViewport,
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
