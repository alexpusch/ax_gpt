use crate::open_api_models::{OpenAiRequestBody, OpenAiResponse, SseChunk};
use async_stream::stream;
use eventsource_client::{Client as EventSourceClient, ClientBuilder, SSE};
use futures::stream::BoxStream;
use futures::stream::StreamExt;

pub struct Client {
    // client: reqwest::blocking::Client,
    api_key: String,
}

impl Client {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }

    pub fn send(&self, request: OpenAiRequestBody) -> reqwest::Result<OpenAiResponse> {
        let body = serde_json::to_string(&request).unwrap();

        log::debug!("Sending request: {:?}", request);
        let body = reqwest::blocking::Client::new()
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .body(body)
            .send()?;

        Ok(serde_json::from_str(&body.text().unwrap()).unwrap())
    }

    pub fn stream(
        &self,
        request: OpenAiRequestBody,
    ) -> eventsource_client::Result<BoxStream<String>> {
        let body = serde_json::to_string(&request).unwrap();

        let client = ClientBuilder::for_url("https://api.openai.com/v1/chat/completions")?
            .method("POST".into())
            .header("Authorization", &format!("Bearer {}", self.api_key))?
            .header("Content-Type", "application/json")?
            .body(body)
            .build();

        let mut stream = client.stream();

        let token_stream = stream! {
            while let Some(event) = stream.next().await {
                match event {
                    Ok(event) => match event {
                        SSE::Comment(comment) => {
                            println!("got a comment event: {:?}", comment);
                        }
                        SSE::Event(event) => {
                            let data: SseChunk = serde_json::from_str(&event.data).expect("valid json");
                            let finish_reason = data.choices[0].finish_reason.clone();

                            if finish_reason == Some("stop".to_string()) {
                                break;
                            }

                            let value = data.choices[0]
                                    .delta
                                    .content
                                    .clone()
                                    .unwrap_or(" ".into())
                                    .clone();

                            yield value;

                        }
                    },
                    Err(error) => {
                        print!("Error from SSR: {:?}", error);
                    }
                }
            }
        };

        Ok(Box::pin(token_stream))
    }
}
