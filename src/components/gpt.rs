use bevy::log;
use bevy::prelude::*;
use chatgpt::prelude::*;
use std::collections::VecDeque;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::constants::bot::*;

use super::action::Action;

/// Represents a conversation with the ChatGPT model.
#[derive(Clone)]
struct GPTConversation {
    client: ChatGPT,
    history: Vec<String>,
    busy: Arc<AtomicBool>,
}

impl GPTConversation {
    /// Creates a new `GPTConversation` with the provided ChatGPT client.
    fn new(client: ChatGPT) -> Self {
        Self {
            client,
            history: Vec::new(),
            busy: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Sends a message to ChatGPT and retrieves the corresponding actions.
    async fn send_message_get_actions(&self, message: &str) -> Option<Vec<Action>> {
        if self.busy.swap(true, Ordering::Acquire) {
            return None;
        }

        log::debug!("Sending:\n{}", message);
        let actions = match self.client.send_message(message).await {
            Ok(response) => {
                let response_txt = &response.message().content;
                log::debug!("Received:\n{}", response_txt);
                Action::from_command_string(response_txt)
            }
            Err(e) => {
                log::warn!("Cannot get GPT answer: {}", e);
                None
            }
        };

        self.busy.store(false, Ordering::Release);

        actions
    }

    /// Retrieves actions with extra context.
    async fn get_actions_with_extra_context(&self, context: &str) -> Option<Vec<Action>> {
        let mut message = self.history.join("\n");
        message.push_str(context);
        self.send_message_get_actions(&message).await
    }

    /// Adds context to the conversation's history.
    fn add_context(&mut self, message: String) {
        log::debug!("Adding to context:\n\"{}\"", message);
        self.history.push(message);
    }
}

/// Component representing a GPT-based agent.
#[derive(Component)]
pub struct GPTAgent {
    conversation: GPTConversation,
    pub action_queue: Arc<Mutex<VecDeque<Action>>>,
}

impl GPTAgent {
    /// Creates a new GPTAgent with the provided API key.
    pub fn new(key: &str) -> Option<Self> {
        let config = ModelConfigurationBuilder::default()
            .engine(ChatGPTEngine::Custom(MODEL))
            .build()
            .unwrap_or_else(|_| panic!("Unable to create GPT config with engine {}!", MODEL));

        let result = ChatGPT::new_with_config(key, config);

        match result {
            Ok(client) => Some(Self {
                conversation: GPTConversation::new(client),
                action_queue: Arc::new(Mutex::new(VecDeque::new())),
            }),
            Err(e) => {
                log::warn!("Cannot create ChatGPT client: {}", e);
                None
            }
        }
    }

    /// Creates actions with extra context from a message.
    pub fn create_actions_with_extra_context(&self, message: &str) {
        let queue_arc = Arc::clone(&self.action_queue);
        let conversation = self.conversation.clone();
        let message_clone = "\n".to_string() + message;

        async_global_executor::spawn(async move {
            if let Ok(mut queue) = queue_arc.try_lock() {
                if let Some(actions) = conversation
                    .get_actions_with_extra_context(&message_clone)
                    .await
                {
                    queue.extend(actions);
                }
            }
        })
        .detach(); // Detach & forget.
    }

    /// Adds context to the conversation.
    pub fn add_context(&mut self, message: &str) {
        self.conversation.add_context(message.to_string());
    }

    /// Checks if the GPT agent is busy.
    pub fn is_busy(&self) -> bool {
        self.conversation.busy.load(Ordering::Relaxed)
    }
}
