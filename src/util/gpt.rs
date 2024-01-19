use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION},
    Client, ClientBuilder, Error,
};
use serde::{Deserialize, Deserializer, Serialize};
use std::time::Duration;

/// Represents a ChatGPT client with an HTTP client and model configuration.
#[derive(Debug, Clone)]
pub struct ChatGPT {
    client: Client,
    pub config: ModelConfiguration,
}

/// Represents a message in a chat, including the role of the sender and the content.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: Role,
    #[serde(deserialize_with = "deserialize_maybe_null")]
    pub content: String,
}

/// Represents the usage of tokens in a request, including prompt, completion, and total tokens.
#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct TokenUsage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

/// Represents a request for a completion from the GPT model.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct CompletionRequest<'a> {
    pub model: &'a str,
    pub messages: &'a Vec<ChatMessage>,
    pub stream: bool,
    pub temperature: f32,
    pub top_p: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    pub frequency_penalty: f32,
    pub presence_penalty: f32,
    #[serde(rename = "n")]
    pub reply_count: u32,
}

/// Represents the response to a completion request, including message ID, timestamp, model info, and choices.
#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct CompletionResponse {
    #[serde(rename = "id")]
    pub message_id: Option<String>,
    #[serde(rename = "created")]
    pub created_timestamp: Option<u64>,
    pub model: String,
    pub usage: TokenUsage,
    #[serde(rename = "choices")]
    pub message_choices: Vec<MessageChoice>,
}

/// Configuration for the ChatGPT model including various parameters and settings.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ModelConfiguration {
    pub engine: String,
    pub temperature: f32,
    pub top_p: f32,
    pub max_tokens: Option<u32>,
    pub presence_penalty: f32,
    pub frequency_penalty: f32,
    pub reply_count: u32,
    pub api_url: String,
    pub timeout: Duration,
}

/// Represents a single choice in a completion response, including the message and reason for finish.
#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct MessageChoice {
    pub message: ChatMessage,
    pub finish_reason: String,
    pub index: u32,
}

/// Role of the sender in a chat message.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Eq, Ord)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    Assistant,
    User,
    Function,
}

/// Represents a server response which can be either an error or a completion.
#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
#[serde(untagged)]
pub enum ServerResponse {
    Error { error: CompletionError },
    Completion(CompletionResponse),
}

/// Details of an error in a completion response.
#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct CompletionError {
    pub message: String,
    #[serde(rename = "type")]
    pub error_type: String,
}

impl Default for ModelConfiguration {
    fn default() -> Self {
        Self {
            engine: Default::default(),
            temperature: 0.5,
            top_p: 1.0,
            max_tokens: None,
            presence_penalty: 0.0,
            frequency_penalty: 0.0,
            reply_count: 1,
            api_url: "https://api.openai.com/v1/chat/completions".into(),
            timeout: Duration::from_secs(10),
        }
    }
}

impl CompletionResponse {
    pub fn message(&self) -> &ChatMessage {
        &self.message_choices.first().unwrap().message
    }
}

impl ChatGPT {
    pub fn new<S: Into<String>>(api_key: S, config: ModelConfiguration) -> Result<Self, Error> {
        let api_key = api_key.into();
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_bytes(format!("Bearer {api_key}").as_bytes())
                .expect("Cannot create HeaderValue"),
        );
        let client = ClientBuilder::new().default_headers(headers).build()?;
        Ok(Self { client, config })
    }

    pub async fn send_message<S: Into<String>>(
        &self,
        message: S,
        role: Role,
    ) -> Result<CompletionResponse, Error> {
        let response = self
            .client
            .post(self.config.api_url.clone())
            .json(&CompletionRequest {
                model: self.config.engine.as_ref(),
                messages: &vec![ChatMessage {
                    role,
                    content: message.into(),
                }],
                stream: false,
                temperature: self.config.temperature,
                top_p: self.config.top_p,
                max_tokens: self.config.max_tokens,
                frequency_penalty: self.config.frequency_penalty,
                presence_penalty: self.config.presence_penalty,
                reply_count: self.config.reply_count,
            })
            .send()
            .await?;
        response.json::<CompletionResponse>().await
    }
}

fn deserialize_maybe_null<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let buf = Option::<String>::deserialize(deserializer)?;
    Ok(buf.unwrap_or_default())
}
