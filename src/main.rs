use std::{env, path::PathBuf};

use bat::PrettyPrinter;
use open_api_models::{Gpt3Role, Message, OpenAiRequestBody, OpenAiResponse, OpenApiModel};
use session_storage::SessionStorage;

mod open_api_models;
mod session_storage;

fn main() {
    env_logger::init();

    let prompt = std::env::args().skip(1).collect::<Vec<String>>().join(" ");
    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");

    let session_storage = SessionStorage::new(PathBuf::from("/tmp/sessions"));
    let mut session = session_storage.get().expect("failed to load session");

    session.messages.push(Message {
        role: Gpt3Role::User,
        content: prompt.to_string(),
    });

    let body = OpenAiRequestBody {
        model: OpenApiModel::Gpt3_5Turbo,
        messages: session.messages.clone(),
    };

    let body = serde_json::to_string(&body).unwrap();

    let client = reqwest::blocking::Client::new();
    let body = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .unwrap();

    let response: OpenAiResponse = serde_json::from_str(&body.text().unwrap()).unwrap();

    session.messages.push(Message {
        role: Gpt3Role::Assistant,
        content: response.choices[0].message.content.clone(),
    });

    session_storage
        .save(&session)
        .expect("failed to save session");

    PrettyPrinter::new()
        .input_from_bytes(response.choices[0].message.content.as_bytes())
        .language("markdown")
        .print()
        .unwrap();
}
