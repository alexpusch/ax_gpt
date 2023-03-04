use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::open_api_models::Message;

#[derive(Debug, Serialize, Deserialize)]
pub struct Session {
    pub messages: Vec<Message>,
}

impl Session {
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
        }
    }
}

pub struct SessionStorage {
    path: PathBuf,
}

impl SessionStorage {
    pub fn new(path: PathBuf) -> Self {
        std::fs::create_dir_all(&path).expect("failed to create session storage directory");

        Self { path }
    }

    pub fn get(&self) -> Result<Session, std::io::Error> {
        let path = self.path.join("session.json");
        let session = if let Ok(file) = std::fs::File::open(path) {
            let reader = std::io::BufReader::new(file);
            let session = serde_json::from_reader(reader)?;

            session
        } else {
            Session::new()
        };

        Ok(session)
    }

    pub fn save(&self, session: &Session) -> Result<(), std::io::Error> {
        let path = self.path.join("session.json");
        let file = std::fs::File::create(path)?;
        let writer = std::io::BufWriter::new(file);
        serde_json::to_writer_pretty(writer, session)?;

        Ok(())
    }
}
