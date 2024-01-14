use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use bevy_ecs_tilemap::prelude::*;
use bevy_pixel_camera::PixelCameraPlugin;
use components::map::{ChunkMap, ChunkSpawningChannel, MainTilemapTexture};
use constants::action::ACTION_TICK_FREQUENCY;
use constants::map::RENDER_CHUNK_SIZE;
use dotenv::dotenv;

mod bundles;
mod components;
mod constants;
mod systems;
mod util;

fn main() {
    dotenv().ok();

    // Setup & Start bevy.
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(LogPlugin {
                    filter: "info,wgpu_core=warn,wgpu_hal=warn,pixel_pal=info".into(),
                    level: bevy::log::Level::DEBUG,
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("PixelPal"),
                        ..Default::default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_plugins(PixelCameraPlugin)
        .add_plugins(LogDiagnosticsPlugin::default())
        .insert_resource(TilemapRenderSettings {
            render_chunk_size: RENDER_CHUNK_SIZE,
            ..Default::default()
        })
        .insert_resource(MainTilemapTexture::default())
        .insert_resource(ChunkMap::new())
        .insert_resource(ChunkSpawningChannel::new())
        .add_plugins(TilemapPlugin)
        .add_systems(Startup, systems::setup::setup)
        .add_systems(Update, systems::input::handle_input)
        .add_systems(Update, systems::input::handle_bot_input)
        .add_systems(Update, systems::chunk::create_chunk_tasks)
        .add_systems(Update, systems::chunk::fetch_chunk_tasks)
        .add_systems(Update, systems::chunk::handle_chunk_despawning)
        .add_systems(Update, systems::animation::animate_sprite)
        .add_systems(Update, systems::bot::query_bot)
        .add_systems(
            Update,
            systems::movement::move_characters.run_if(on_timer(ACTION_TICK_FREQUENCY)),
        )
        .add_systems(
            Update,
            systems::movement::camera_follow_player.after(systems::movement::move_characters),
        )
        .run();
}
