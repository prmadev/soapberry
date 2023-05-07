//! persisted events in the structsy store

pub mod entry_was_created;

use thiserror::Error;
use whirlybird::journey::DomainError;

/// Public error type related to converting service
#[derive(Error, Debug, Clone)]
pub enum ToPersistenceConversionError {
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

/// Public error type related to converting service
#[derive(Error, Debug, Clone)]
pub enum FromPersistenceConversionError {
    /// errors related to parsing of UUID from string
    #[error("ID cannot get parsed: {0}")]
    CouldNotBuildUUID(#[from] uuid::Error),

    /// errors related to title creation from input
    #[error("Title cannot get built: {0}")]
    TitleCannotBeBuilt(whirlybird::journey::entity::title::BuildingError),

    /// errors related to body creation from input
    #[error("Body cannot get built: {0}")]
    BodyCannotBeBuilt(whirlybird::journey::entity::body::BuildingError),

    /// not a correct event type
    #[error("Incorrect event type")]
    IncorrectEventType,

    /// time could not be parsed into u8
    #[error("Time could not be parsed from u64")]
    TimeCouldNotBeParsedFromU64(u64),
}
