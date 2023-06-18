//! configurations

//
// # type declaration
//

use std::{fs, path::PathBuf};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub file_store: Option<PathBuf>,
}

//
// # implementations
//

impl From<crate::cli::Args> for Config {
    fn from(value: crate::cli::Args) -> Self {
        Self {
            file_store: value.file_store,
        }
    }
}

impl TryFrom<PathBuf> for Config {
    type Error = ConfigFileGenerationError;

    fn try_from(config_file: PathBuf) -> color_eyre::Result<Self, Self::Error> {
        Ok(serde_json::from_slice::<Config>(&fs::read(config_file)?)?)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigFileGenerationError {
    #[error("could not read file{0}")]
    CouldNotReadFile(#[from] std::io::Error),
    #[error("could not read file{0}")]
    ParsingConfigFileFailed(#[from] serde_json::Error),
}
