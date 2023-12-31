use bevy::prelude::*;
use bevy_pixel_camera::*;
use crate::bundles::map::MapBundle;
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

    // Spawn Map
    commands.spawn(MapBundle::new());

    // Spawn Player
    commands.spawn(
        PlayerBundle::new(USER_SPAWN, &asset_server, &mut textures)
    ).insert(IsUser);

    // Spawn Mittens (GPT)
    commands.spawn(
        PlayerBundle::new(MITTENS_SPAWN, &asset_server, &mut textures)
    ).insert(IsBot);
}
