use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use components::mapping::ChunkManager;
use constants::mapping::RENDER_CHUNK_SIZE;
use dotenv::dotenv;

mod util;
mod systems;
mod components;
mod bundles;
mod generation;
mod constants;


#[tokio::main]
async fn main(){
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
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: String::from("TestGame"),
                    ..Default::default()
                }),
                ..default()
            }),
        )
        .insert_resource(TilemapRenderSettings {
            render_chunk_size: RENDER_CHUNK_SIZE,
            ..Default::default()
        })
        .add_plugins(TilemapPlugin)
        .insert_resource(ChunkManager::default())
        .add_systems(Startup, systems::setup::setup)
        .add_systems(Update, systems::movement::movement)
        //.add_systems(Update, systems::animation::animate_sprite)
        .add_systems(Update, systems::chunk::handle_chunk_spawning)
        .add_systems(Update, systems::chunk::handle_chunk_despawning)
        .run();
}
