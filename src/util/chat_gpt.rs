use chatgpt::prelude::*;
use bevy::log::*;

use super::messages;


pub fn new_client(key: &str) -> Result<ChatGPT> {
    let config = ModelConfigurationBuilder::default()
        .engine(ChatGPTEngine::Custom("gpt-4-1106-preview"))
        .build()
        .unwrap(); // We're sure this won't produce any error.

    ChatGPT::new_with_config(key, config)
}

pub async fn send_message(mut conversation: Conversation, message: &str) {
    // Sending a message and getting the completion
    let response = conversation
        .send_message(message)
        .await
        .unwrap();

    info!("{}", response.message().content);
}

pub async fn send_instructions(conversation: Conversation) {
    send_message(conversation, &messages::INSTRUCTIONS).await;
}
