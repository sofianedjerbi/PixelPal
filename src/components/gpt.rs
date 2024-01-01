use std::sync::Arc;
use std::collections::VecDeque;
use bevy::log;
use bevy::prelude::*;
use chatgpt::prelude::*;
use tokio::sync::Mutex;

use crate::constants::gpt::*;

use super::action::Action;

#[derive(Clone)]
struct GPTConversation {
    client: ChatGPT,
    history: Vec<String>
}

impl GPTConversation {
    fn new(client: ChatGPT) -> Self {
        Self {
            client,
            history: Vec::new()
        }
    }

    async fn get_actions(
        self
    ) -> Option<Vec<Action>> {
        let message = self.history.join("\n");
        log::info!("Sending:\n\"{}\"", message);
        match self.client.send_message(message).await {
            Ok(response) => {
                let response_txt = &response.message().content;
                log::info!("Received:\n\"{}\"", &response_txt);
                // Eventually save the answer here (if the agent is using talk command)
                Action::from_command_string(&response_txt)
            },
            Err(e) => {
                log::warn!("Cannot get GPT answer: {}", e);
                None
            }
        }
    }

    fn add_context(
        &mut self,
        message: String
    ) {
        log::info!("Adding to context:\n\"{}\"", message);
        self.history.push(message);
    }
}


#[derive(Component)]
pub struct GPTAgent {
    conversation: GPTConversation,
    pub action_queue: Arc<Mutex<VecDeque<Action>>>
}

impl GPTAgent {
    pub fn new(key: &str) -> Option<Self> {
        let config = ModelConfigurationBuilder::default()
        .engine(ChatGPTEngine::Custom(MODEL))
        .build()
        .unwrap(); // We're sure this won't produce any error.

        let result = ChatGPT::new_with_config(key, config);
        
        match result {
            Ok(client) => {
                Some(
                    Self {
                        conversation: GPTConversation::new(client),
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

    pub fn create_actions(&self) {
        let queue_arc = Arc::clone(&self.action_queue);
        let conversation = self.conversation.clone();

        async_global_executor::spawn(async move {
            if let Ok(mut queue) = queue_arc.try_lock() {
                if let Some(actions) = conversation.get_actions().await {
                    queue.extend(actions);
                }
            }
        }).detach(); // Detach & forget.
    }

    pub fn add_context(
        &mut self,
        message: &str
    ) {
        self.conversation.add_context(message.to_string());
    }
}
