//! configurations

//
// # type declaration
//

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub file_store: Option<std::path::PathBuf>,
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
