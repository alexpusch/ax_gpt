use std::io::stdout;
use std::io::Write;
use std::{env, path::PathBuf};

use bat::PrettyPrinter;
use cmd_gpt::open_ai_client;
use cmd_gpt::open_api_models::{Gpt3Role, Message, OpenAiRequestBody, OpenApiModel, SseChunk};
use cmd_gpt::session_storage::SessionStorage;
use futures::StreamExt;

#[tokio::main]
async fn main() {
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
        stream: true,
    };

    let client = open_ai_client::Client::new(api_key);
    let resposne = client.stream(body).expect("failed to send request");

    let full_response = resposne
        .inspect(|s| {
            print!("{}", s);
            stdout().flush().expect("failed to flush stdout");
        })
        .collect::<Vec<_>>()
        .await
        .join(" ");

    session.push_message(Message {
        role: Gpt3Role::Assistant,
        content: full_response,
    });

    session_storage
        .save(&session)
        .expect("failed to save session");

    // PrettyPrinter::new()
    //     .input_from_bytes(response.choices[0].message.content.as_bytes())
    //     .language("markdown")
    //     .print()
    //     .unwrap();
}
