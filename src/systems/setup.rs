use bevy::prelude::*;
use crate::bundles::player::PlayerBundle;
use crate::components::animation::{SpriteAnimation, SpriteAnimationState};
use crate::constants::display::ZOOM;


const _ZOOM_VALUE: f32 = 1. / ZOOM;

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    textures: ResMut<Assets<TextureAtlas>>
 ) {
    // Spawn the camera
    commands.spawn(Camera2dBundle {
        transform: Transform::from_scale(Vec3::new(_ZOOM_VALUE, _ZOOM_VALUE, 1.)), // Directly set scale
        ..Default::default()
    });

    // Spawn player
    /*commands.spawn((
        PlayerBundle::new(asset_server, textures),
        SpriteAnimationState::default(),
        SpriteAnimation::new([2,3], 3.)
    ));*/
}
