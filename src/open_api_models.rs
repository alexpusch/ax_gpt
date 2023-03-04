pub use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum OpenApiModel {
    #[serde(rename = "gpt-3.5-turbo")]
    Gpt3_5Turbo,
}

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
    pub model: OpenApiModel,
    pub messages: Vec<Message>,
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
