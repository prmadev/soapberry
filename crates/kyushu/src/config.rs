//! Configurations Management

use std::{fs, path::PathBuf};

/// Represents the configuration of the program.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct Config {
    /// Holds the absolute path to the repository.
    pub file_store: Option<PathBuf>,
}

/// Holder for inputs in different formats.
#[derive(Debug, Clone)]
pub struct InputInfo {
    args: Option<crate::cli::Args>,
    file: Option<Config>,
}

impl InputInfo {
    /// Creates a new instance of `InputInfo`.
    #[must_use]
    pub const fn new(args: Option<crate::cli::Args>, file: Option<Config>) -> Self {
        Self { args, file }
    }
}

impl From<InputInfo> for Config {
    fn from(value: InputInfo) -> Self {
        // setting the defaults first
        let mut res = Self {
            file_store: Option::default(),
        };

        // changing the defaults if there is arguments
        if let Some(cf) = value.file {
            if let Some(cf_file_store) = cf.file_store {
                res.file_store = Some(cf_file_store);
            };
        };

        // overwriting defaults and config file if arguments are passed
        if let Some(arg_input) = value.args {
            if let Some(cf_file_store) = arg_input.file_store {
                res.file_store = Some(cf_file_store);
            };
        };

        res
    }
}

impl TryFrom<PathBuf> for Config {
    type Error = FileGenerationError;

    fn try_from(config_file: PathBuf) -> color_eyre::Result<Self, Self::Error> {
        Ok(serde_json::from_slice::<Self>(&fs::read(config_file)?)?)
    }
}

/// Errors that can occur during the conversion of `PathBuf` to `Config`.
#[derive(Debug, thiserror::Error)]
pub enum FileGenerationError {
    /// Indicates that the file could not be read.
    #[error("Failed to read the file: {0}")]
    FailedToReadFile(#[from] std::io::Error),
    /// Indicates that the configuration file could not be parsed.
    #[error("Failed to parse the configuration file: {0}")]
    FailedToParseConfigFile(#[from] serde_json::Error),
}
