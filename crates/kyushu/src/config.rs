//! configurations

//
// # type declaration
//

use std::{fs, path::PathBuf};

/// the Configuration of the program.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct Config {
    /// [`file_store`] holds the absolute path to the repo.
    pub file_store: Option<PathBuf>,
}

/// holder for inputs of different format
#[derive(Debug, Clone)]
pub struct InputInfo {
    args: Option<crate::cli::Args>,
    file: Option<Config>,
}

impl InputInfo {
    /// Creates a new input Info
    #[must_use]
    pub const fn new(args: Option<crate::cli::Args>, file: Option<Config>) -> Self {
        Self { args, file }
    }
}

//
// # implementations
//

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
