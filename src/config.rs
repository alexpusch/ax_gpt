// https://platform.openai.com/docs/api-reference/chat

use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
};

use config::{Config, ConfigError};
use serde::{Deserialize, Serialize};
use std::io;

use thiserror::Error;

const API_KEY_PLACEHOLDER: &str = "OPENAI API KEY";
const DEFAULT_SYSTEM_PROMPT: &str = "You are a programmers assistant";
const DEFAULT_MODEL: &str = "gpt-3.5-turbo";
const CONFIG_FILE_NAME: &str = "aks_gpt.json";

#[derive(Error, Debug)]
pub enum AksConfigError {
    #[error("Missing Api key")]
    MissingApiKey,

    #[error("Missing Api key")]
    FailedToWriteConfig(#[from] std::io::Error),

    #[error("Config loading error")]
    ConfigError(#[from] ConfigError),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AksConfig {
    pub openai_api_key: String,
    pub system_prompt: String,
    pub model: String,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub temperature: Option<f64>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub top_p: Option<f64>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub max_tokens: Option<u16>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub presence_penalty: Option<f64>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub frequency_penalty: Option<f64>,
}

impl Default for AksConfig {
    fn default() -> Self {
        AksConfig {
            openai_api_key: API_KEY_PLACEHOLDER.into(),
            system_prompt: DEFAULT_SYSTEM_PROMPT.into(),
            model: DEFAULT_MODEL.into(),
        }
    }
}

pub fn get_config() -> Result<AksConfig, AksConfigError> {
    let config_dir = get_config_dir();
    let config_file_path = config_dir.join(CONFIG_FILE_NAME);

    let config_builder = Config::builder()
        .add_source(config::File::from(config_file_path.clone()))
        .set_default("system_prompt", DEFAULT_SYSTEM_PROMPT)
        .expect("config default set")
        .set_default("model", DEFAULT_MODEL)
        .expect("config default set");

    let config = match config_builder.build() {
        Ok(config) => config
            .try_deserialize::<AksConfig>()
            .map_err(AksConfigError::ConfigError)?,
        Err(ConfigError::Foreign(_)) => {
            let default_config = AksConfig::default();
            write_default_config_file(&config_dir, CONFIG_FILE_NAME, &default_config)
                .map_err(AksConfigError::FailedToWriteConfig)?;

            default_config
        }
        Err(e) => return Err(AksConfigError::ConfigError(e)),
    };

    if config.openai_api_key == API_KEY_PLACEHOLDER {
        return Err(AksConfigError::MissingApiKey);
    }

    Ok(config)
}

pub fn get_config_dir() -> PathBuf {
    let home = std::env::var("HOME").expect("must have $Home");
    PathBuf::from(home).join(".config")
}

pub fn get_config_filepath() -> PathBuf {
    get_config_dir().join(CONFIG_FILE_NAME)
}

fn write_default_config_file(
    config_dir: &Path,
    config_filename: &str,
    config: &AksConfig,
) -> io::Result<()> {
    fs::create_dir_all(config_dir)?;
    let mut file = fs::File::create(config_dir.join(config_filename))?;
    file.write_all(&serde_json::to_vec_pretty(config).expect("failed to serialize"))?;

    Ok(())
}
