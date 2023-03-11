use ax_gpt::config::{get_config_filepath, AxConfigError};
use ax_gpt::open_ai_client;
use ax_gpt::open_api_models::{Gpt3Role, Message, OpenAiRequestBody};
use ax_gpt::session_storage::SessionStorage;
use bat::PrettyPrinter;
use futures::StreamExt;
use std::env::temp_dir;

use colored::*;
use iter_read::IterRead;
use std::sync::mpsc;

#[tokio::main]
async fn main() {
    env_logger::init();

    let prompt = std::env::args().skip(1).collect::<Vec<String>>().join(" ");

    let config = ax_gpt::config::get_config();

    let Ok(config) = config else {
        match config {
            Ok(_) => unreachable!(),
            Err(AxConfigError::MissingApiKey) => println!("OpenAI api key is requried. 
create the config file {} and insert your OpenAI api key:

{{  
    \"openai_api_key\": \"OPENAI API KEY\"
}}
    
", get_config_filepath().to_string_lossy().bold()),
            Err(AxConfigError::FailedToWriteConfig(e)) => println!("Failed to write config file: {}", e),
            Err(AxConfigError::ConfigError(e)) => println!("Failed to open config file: {}", e)
        }
        std::process::exit(1);
    };

    let session_storage = SessionStorage::new(temp_dir().join("ax_gpt/sessions"));
    let mut session = session_storage.get().expect("failed to load session");

    session.messages.push(Message {
        role: Gpt3Role::User,
        content: prompt.to_string(),
    });

    let mut session_clone = session.clone();

    // I wanted to make the output stream, but be pretty printed in the same time.
    // To achive this I get the tokens in an async spawn, send them back via std::sync channel
    // than use IterRead to convert it into Reader and shove this thing into PrettyPrinter.
    // Luckly PrettyPrinter reads the Reader line by line. Phew...
    let (tx, rx) = mpsc::channel::<String>();
    tokio::spawn(async move {
        if config.system_prompt.len() > 0 {
            session_clone.messages.insert(
                0,
                Message {
                    role: Gpt3Role::System,
                    content: config.system_prompt,
                },
            );
        }

        let body = OpenAiRequestBody {
            model: config.model,
            messages: session_clone.messages.clone(),
            stream: true,
        };

        let client = open_ai_client::Client::new(config.openai_api_key);
        let mut resposne = client.stream(body).expect("failed to send request");

        while let Some(s) = resposne.next().await {
            tx.send(s).expect("failed to send");
        }
    });

    let mut tokens = Vec::<String>::new();
    let token_reader = IterRead::new(rx.iter().fuse().inspect(|v| tokens.push(v.clone())));

    PrettyPrinter::new()
        .input_from_reader(token_reader)
        .language("markdown")
        .print()
        .unwrap();

    let full_response = tokens.join("");
    session.push_message(Message {
        role: Gpt3Role::Assistant,
        content: full_response,
    });

    session.trim(4);

    session_storage
        .save(&session)
        .expect("failed to save session");
}
