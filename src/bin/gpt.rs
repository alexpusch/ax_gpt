use std::{env, path::PathBuf};

use bat::PrettyPrinter;
use cmd_gpt::open_ai_client;
use cmd_gpt::open_api_models::{Gpt3Role, Message, OpenAiRequestBody, OpenApiModel};
use cmd_gpt::session_storage::SessionStorage;
use futures::StreamExt;

use iter_read::IterRead;
use std::sync::mpsc;

#[tokio::main]
async fn main() {
    env_logger::init();

    let prompt = std::env::args().skip(1).collect::<Vec<String>>().join(" ");
    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");

    let session_storage = SessionStorage::new(PathBuf::from("/tmp/sessions"));
    let mut session = session_storage.get().expect("failed to load session");
    let mut session_clone = session.clone();
    let (tx, rx) = mpsc::channel::<String>();

    // I wanted to make the output stream, but be pretty printed in the same time.
    // To achive this I get the tokens in an async spawn, send them back via std::sync channel
    // than use IterRead to convert it into Reader and shove this thing into PrettyPrinter.
    // Luckly PrettyPrinter reads the Reader line by line. Phew...
    tokio::spawn(async move {
        session_clone.messages.push(Message {
            role: Gpt3Role::User,
            content: prompt.to_string(),
        });

        let body = OpenAiRequestBody {
            model: OpenApiModel::Gpt3_5Turbo,
            messages: session_clone.messages.clone(),
            stream: true,
        };

        let client = open_ai_client::Client::new(api_key);
        let mut resposne = client.stream(body).expect("failed to send request");

        while let Some(s) = resposne.next().await {
            tx.send(s).expect("failed to send");
        }
    });

    let mut tokens = Vec::<String>::new();
    let r = IterRead::new(rx.iter().fuse().inspect(|v| tokens.push(v.clone())));

    PrettyPrinter::new()
        .input_from_reader(r)
        .language("markdown")
        .print()
        .unwrap();

    let full_response = tokens.join("");
    session.push_message(Message {
        role: Gpt3Role::Assistant,
        content: full_response,
    });

    session_storage
        .save(&session)
        .expect("failed to save session");
}
