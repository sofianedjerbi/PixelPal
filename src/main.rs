use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use bevy_ecs_tilemap::prelude::*;
use bevy_pixel_camera::PixelCameraPlugin;
use components::mapping::IndexChunkList;
use constants::gpt::SEND_MAP_FREQUENCY;
use constants::mapping::RENDER_CHUNK_SIZE;
use constants::action::ACTION_TICK_FREQUENCY;
use dotenv::dotenv;

mod systems;
mod components;
mod bundles;
mod generation;
mod constants;
mod events;



fn main(){
    // setup program
    dotenv().ok();

    // get API, load client & get client ready
    //let api_key = env::var("GPT_KEY").expect("GPT_KEY not found");
    //let client = ai::chat_gpt::new_client(&api_key).unwrap();
    //let conversation = client.new_conversation();
    //ai::chat_gpt::send_instructions(conversation).await;

    // run app
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
        .add_systems(Update, systems::input::handle_input)
        .add_systems(Update, systems::input::handle_bot_input)
        .add_systems(Update, systems::animation::animate_sprite)
        .add_systems(
            Update,
            systems::movement::move_characters
                .run_if(on_timer(ACTION_TICK_FREQUENCY))
        )
        .add_systems(
            Update,
            systems::bot::send_map_to_bot
                .run_if(on_timer(SEND_MAP_FREQUENCY))
        )
        .add_systems(
            Update, 
            systems::movement::camera_follow_player
                .after(systems::movement::move_characters)
        )
        .add_systems(Update, systems::chunk::handle_chunk_spawning)
        .add_systems(Update, systems::chunk::handle_chunk_despawning)
        .insert_resource(IndexChunkList::new())
        .run();
}
