use async_compat::Compat;
use bevy::log;
use bevy::prelude::*;
use bevy::tasks::AsyncComputeTaskPool;
use std::collections::VecDeque;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::util::gpt::*;

use super::action::Action;

/// Represents a conversation with the ChatGPT model.
#[derive(Clone)]
struct GPTConversation {
    client: ChatGPT,
    context: Vec<String>,
    busy: Arc<AtomicBool>,
}

/// Component representing a GPT-based agent.
#[derive(Component)]
pub struct GPTAgent {
    conversation: Arc<RwLock<GPTConversation>>,
    pub action_queue: Arc<RwLock<VecDeque<Action>>>,
}

impl GPTConversation {
    /// Creates a new `GPTConversation` with the provided ChatGPT client.
    fn new(client: ChatGPT) -> Self {
        Self {
            client,
            context: Vec::new(),
            busy: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Sends a message to ChatGPT and retrieves the corresponding actions.
    async fn send_message_get_actions(&self, message: &str) -> Option<Vec<Action>> {
        if self.busy.swap(true, Ordering::Acquire) {
            return None;
        }

        log::debug!("Sending:\n{}", message);
        let actions = match self.client.send_message(message, Role::System).await {
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
        let mut message = self.context.join("\n");
        message.push_str(context);
        self.send_message_get_actions(&message).await
    }

    /// Adds context to the conversation's history.
    fn add_context(&mut self, message: String) {
        log::debug!("Adding to context:\n\"{}\"", message);
        self.context.push(message);
    }
}

impl GPTAgent {
    /// Creates a new GPTAgent with the provided API key.
    pub fn new(key: String, model: String, url: String) -> Option<Self> {
        let config = ModelConfiguration {
            engine: model,
            api_url: url,
            ..Default::default()
        };

        let result = ChatGPT::new(key, config);

        match result {
            Ok(client) => Some(Self {
                conversation: Arc::new(RwLock::new(GPTConversation::new(client))),
                action_queue: Arc::new(RwLock::new(VecDeque::new())),
            }),
            Err(e) => {
                log::warn!("Cannot create ChatGPT client: {}", e);
                None
            }
        }
    }

    /// Creates actions with extra context from a message.
    pub fn create_actions_with_extra_context(&self, message: &str) {
        let queue_arc = self.action_queue.clone();
        let conversation_arc = self.conversation.clone();
        let message = "\n".to_string() + message;
        let thread_pool = AsyncComputeTaskPool::get();

        thread_pool
            .spawn(Compat::new(async move {
                if let Ok(mut queue) = queue_arc.try_write() {
                    if let Ok(conversation) = conversation_arc.try_read() {
                        if let Some(actions) =
                            conversation.get_actions_with_extra_context(&message).await
                        {
                            queue.extend(actions);
                        }
                    }
                }
            }))
            .detach(); // Detach & forget.
    }

    /// Adds context to the conversation.
    pub fn add_context(&mut self, message: &str) {
        if let Ok(mut conversation) = self.conversation.try_write() {
            conversation.add_context(message.to_string());
        }
    }

    /// Checks if the GPT agent is busy.
    pub fn is_busy(&self) -> bool {
        if let Ok(conversation) = self.conversation.try_read() {
            return conversation.busy.load(Ordering::Relaxed);
        }
        false
    }
}
