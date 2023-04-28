//! persisted events in the structsy store

pub mod entry_was_created;
pub mod journey_was_created;

use thiserror::Error;
use whirlybird::journey::DomainError;

/// Public error type related to converting service
#[derive(Error, Debug, Clone)]
pub enum PersistenceConversionError {
    /// errors related to parsing of UUID from string
    #[error("ID cannot get parsed: {0}")]
    CouldNotBuildUUID(#[from] uuid::Error),

    /// errors related to title or body creation from input
    #[error("Title/body cannot get built: {0}")]
    TitleOrBodyCannotBeBuilt(#[from] DomainError),

    /// not a correct event type
    #[error("Incorrect event type")]
    IncorrectEventType,

    /// time could not be parsed into u8
    #[error("Time could not be parsed into u8")]
    TimeCouldNotBeParsedIntoU8,
}
