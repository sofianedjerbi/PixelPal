use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use bevy_ecs_tilemap::prelude::*;
use bevy_pixel_camera::PixelCameraPlugin;
use components::map::ChunkMap;
use constants::mapping::RENDER_CHUNK_SIZE;
use constants::action::ACTION_TICK_FREQUENCY;
use dotenv::dotenv;

mod systems;
mod components;
mod bundles;
mod util;
mod constants;
mod events;



fn main(){
    dotenv().ok();

    App::new()
        .add_plugins(
            DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(LogPlugin {
                filter: "info,wgpu_core=warn,wgpu_hal=warn,test=debug".into(),
                level: bevy::log::Level::DEBUG,
            })
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: String::from("TestGame"),
                    ..Default::default()
                }),
                ..default()
            }),
        )
        .add_plugins(PixelCameraPlugin)
        .insert_resource(TilemapRenderSettings {
            render_chunk_size: RENDER_CHUNK_SIZE,
            ..Default::default()
        })
        .add_plugins(TilemapPlugin)
        .add_systems(Startup, systems::setup::setup)
        .add_systems(PostStartup, systems::chunk::handle_chunk_spawning)
        .add_systems(Update, systems::input::handle_input)
        .add_systems(Update, systems::input::handle_bot_input)
        .add_systems(Update, systems::chunk::handle_chunk_spawning)
        .add_systems(Update, systems::chunk::handle_chunk_despawning)
        .add_systems(Update, systems::animation::animate_sprite)
        .add_systems(
            Update,
            systems::movement::move_characters
                .run_if(on_timer(ACTION_TICK_FREQUENCY))
        )
        .add_systems(
            Update,
            systems::bot::send_map_to_bot
                .after(systems::chunk::handle_chunk_spawning)
        )
        .add_systems(
            Update, 
            systems::movement::camera_follow_player
                .after(systems::movement::move_characters)
        )
        .insert_resource(ChunkMap::new())
        .run();
}
