use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, time::Duration};

use crate::open_api_models::Message;

const SESSION_TTL: Duration = Duration::from_secs(5 * 60);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Session {
    pub created_at: DateTime<Utc>,
    pub messages: Vec<Message>,
}

impl Session {
    pub fn new() -> Self {
        Self {
            created_at: chrono::Utc::now(),
            messages: Vec::new(),
        }
    }

    pub fn push_message(&mut self, message: Message) {
        self.messages.push(message);
    }

    pub fn trim(&mut self, depth: usize) {
        self.messages = self
            .messages
            .iter()
            .rev()
            .take(depth)
            .rev()
            .cloned()
            .collect();
    }
}

pub struct SessionStorage {
    path: PathBuf,
}

impl SessionStorage {
    pub fn new(path: PathBuf) -> Self {
        let session_id = get_current_session_id();
        log::debug!("Storing session in {:?}, Session ID: {}", path, session_id);
        std::fs::create_dir_all(&path).expect("failed to create session storage directory");

        let filename = format!("{}.json", session_id);
        Self {
            path: path.join(filename),
        }
    }

    pub fn get(&self) -> Result<Session, std::io::Error> {
        let session = if let Ok(file) = std::fs::File::open(&self.path) {
            let reader = std::io::BufReader::new(file);
            let session: Session = serde_json::from_reader(reader)?;

            if session.created_at + chrono::Duration::from_std(SESSION_TTL).unwrap() < Utc::now() {
                log::debug!("Session expired, creating new one");
                Session::new()
            } else {
                session
            }
        } else {
            Session::new()
        };

        Ok(session)
    }

    pub fn save(&self, session: &Session) -> Result<(), std::io::Error> {
        let file = std::fs::File::create(&self.path)?;
        let writer = std::io::BufWriter::new(file);
        serde_json::to_writer_pretty(writer, session)?;

        Ok(())
    }
}

fn get_current_session_id() -> String {
    let session_id = unsafe { libc::getppid() };
    session_id.to_string()
}
