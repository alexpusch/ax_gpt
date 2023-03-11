use crate::open_api_models::{OpenAiRequestBody, OpenAiResponse, SseChunk};
use async_stream::stream;
use eventsource_client::{Client as EventSourceClient, ClientBuilder, SSE};
use futures::stream::BoxStream;
use futures::stream::StreamExt;

pub struct Client {
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
        log::debug!("Sending request: {:?}", request);

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
                        SSE::Comment(_) => {
                            // should not happened
                        }
                        SSE::Event(event) => {
                            let data: SseChunk = serde_json::from_str(&event.data).expect("invalid json when deserializing");
                            let finish_reason = data.choices[0].finish_reason.clone();

                            if finish_reason == Some("stop".to_string()) {
                                yield "\n".to_string();
                                break;
                            }

                            let value = data.choices[0]
                                    .delta
                                    .content
                                    .clone()
                                    .unwrap_or("".into());

                            yield value;

                        }
                    },
                    Err(eventsource_client::Error::UnexpectedResponse(status)) if status == 401 => {
                        println!("Unautherized OpenAI API key: {:?}", status);
                        break;
                    },
                    Err(error) => {
                        println!("Unexpected server error: {:?}", error);
                        break;
                    }
                }
            }
        };

        Ok(Box::pin(token_stream))
    }
}
