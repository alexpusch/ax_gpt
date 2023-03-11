pub use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Gpt3Role {
    User,
    Assistant,
    System,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: Gpt3Role,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAiRequestBody {
    pub model: String,
    pub messages: Vec<Message>,
    pub stream: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAiResponse {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub usage: serde_json::Value,
    pub choices: Vec<OpenAiChoice>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAiChoice {
    pub message: OpenAiMessage,
    pub finish_reason: String,
    pub index: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAiMessage {
    pub role: Gpt3Role,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SseChunk {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<Choice>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Choice {
    pub delta: Delta,
    pub index: u64,
    pub finish_reason: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Delta {
    pub content: Option<String>,
}
