use std::str::FromStr;
use std::sync::Arc;
use std::collections::VecDeque;
use bevy::log;
use bevy::prelude::*;
use chatgpt::prelude::*;
use tokio::sync::Mutex;

use crate::components::action::*;
use crate::constants::gpt::COMMANDS;
use crate::constants::gpt::UNDERSTANDING;
use crate::constants::gpt::WRONG_COMMAND;

use super::action::Action;


#[derive(Component)]
pub struct GPTAgent {
    client: ChatGPT,
    conversation: Arc<Mutex<Conversation>>,
    pub action_queue: Arc<Mutex<VecDeque<Action>>>
}

impl GPTAgent {
    pub fn new(key: &str) -> Option<Self> {
        let config = ModelConfigurationBuilder::default()
        .engine(ChatGPTEngine::Custom("gpt-4-1106-preview"))
        .build()
        .unwrap(); // We're sure this won't produce any error.

        let result = ChatGPT::new_with_config(key, config);
        
        match result {
            Ok(client) => {
                let conversation = client.new_conversation();
                Some(
                    Self {
                        client,
                        conversation: Arc::new(Mutex::new(conversation)),
                        action_queue: Arc::new(Mutex::new(VecDeque::new()))
                    }
                )
            },
            Err(e) => {
                log::warn!("Cannot create ChatGPT client: {}", e);
                return None;
            }
        }
    }

    // TODO: Simplify this crap
    pub fn send_message(&mut self, message: String) {
        let conversation = Arc::clone(&self.conversation);
        let queue = Arc::clone(&self.action_queue);

        async_global_executor::spawn(async move {
            log::info!("Sending: \"{}\"", &message);

            let mut conversation = conversation.lock().await;

            match conversation.send_message(&message).await {
                Ok(response) => {
                    let mut queue = queue.lock().await;
                    let message = &response.message().content.to_uppercase();
                    log::info!("Received: \"{}\"", message);
                    if let Some(action) = command_to_action(message) {
                        queue.push_back(action);
                        true
                    } else {
                        log::info!("Unknown command: {}", message);
                        match conversation.send_message(
                            WRONG_COMMAND.to_string() + COMMANDS + UNDERSTANDING
                        ).await {
                            Ok(response2) => {
                                log::info!(
                                    "Received: \"{}\"",
                                    response2.message().content.to_uppercase()
                                );
                            }
                            Err(e) => {
                                log::warn!("No answer to wrong command: \"{}\"", e);
                            }
                        }
                        false
                    }
                }
                Err(e) => {
                    log::info!("No answer: {}", e);
                    false
                }
            };
        }).detach(); // Detach the task so it runs independently
    }
}

// Util function
fn command_to_action(command: &str) -> Option<Action> {
    let parts: Vec<&str> = command.split_whitespace().collect();

    if parts.len() == 3 && parts[0] == "MOVE" {
        // Use ActionDirection::from_str for conversion
        let direction_result = ActionDirection::from_str(parts[1].to_uppercase().as_str());

        // Handle the case where the direction isn't valid
        let direction = match direction_result {
            Ok(dir) => dir,
            Err(_) => return None, // Early return if not a valid direction
        };

        // Parse the "TIMES" part, assuming you want to do something with it later
        let times_result = parts[2].parse::<usize>();
        if times_result.is_err() {
            return None; // Early return if "TIMES" isn't a valid usize
        }

        Some(Action {
            kind: ActionKind::Walking, // Assuming "MOVE" implies Walking
            direction,
        })
    } else {
        None
    }
}
