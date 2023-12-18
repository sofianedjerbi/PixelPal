mod ai;
mod map;

use bevy::{prelude::*, log::{LogPlugin, Level}};
use bevy_ecs_tilemap::prelude::*;
use dotenv::dotenv;

use std::env;


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
        .add_plugins(DefaultPlugins)
        .add_plugins(TilemapPlugin)
        .run();
}
