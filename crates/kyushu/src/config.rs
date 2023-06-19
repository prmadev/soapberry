//! configurations

//
// # type declaration
//

use std::{fs, path::PathBuf};

/// the Configuration of the program.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Config {
    /// [`file_store`] holds the absolute path to the repo.
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
    type Error = FileGenerationError;

    fn try_from(config_file: PathBuf) -> color_eyre::Result<Self, Self::Error> {
        Ok(serde_json::from_slice::<Self>(&fs::read(config_file)?)?)
    }
}

/// Errors that arise when converting [`PathBuf`] to [`Config`]
#[derive(Debug, thiserror::Error)]
pub enum FileGenerationError {
    /// This error indicates that it could not read the file
    #[error("could not read file{0}")]
    CouldNotReadFile(#[from] std::io::Error),
    /// This error indicates that config file could not be parsed
    #[error("could not read file{0}")]
    ParsingConfigFileFailed(#[from] serde_json::Error),
}
