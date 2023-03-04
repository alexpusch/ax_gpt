use crate::open_api_models::{OpenAiRequestBody, OpenAiResponse};

pub struct Client {
    client: reqwest::blocking::Client,
    api_key: String,
}

impl Client {
    pub fn new(api_key: String) -> Self {
        Self {
            client: reqwest::blocking::Client::new(),
            api_key,
        }
    }

    pub fn send(&self, request: OpenAiRequestBody) -> reqwest::Result<OpenAiResponse> {
        let body = serde_json::to_string(&request).unwrap();

        log::debug!("Sending request: {:?}", request);
        let body = self
            .client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .body(body)
            .send()?;

        Ok(serde_json::from_str(&body.text().unwrap()).unwrap())
    }
}
