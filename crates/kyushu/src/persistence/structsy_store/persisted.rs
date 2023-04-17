//! persisted events in the structsy store

#![allow(missing_docs)]
use std::time::{Duration, UNIX_EPOCH};

use redmaple::id::ID;
use structsy::derive::{queries, Persistent};
use thiserror::Error;
use uuid::Uuid;
use whirlybird::journey::{title::Title, DomainError, Journey};

/// Event Stored for when an event was created
#[derive(Persistent, Clone)]
pub struct JourneyWasCreated {
    title: String,
    journey_id: String,
    time_created: u64,
}

/// queries relatedo to the Object
#[queries(JourneyWasCreated)]
pub trait JourneyWasCreatedQuery {
    /// searches for matching titles
    #[must_use]
    fn search_title(self, title: &str) -> Self;
    /// seraches for specific id
    #[must_use]
    fn search_by_journey_id(self, journey_id: &str) -> Self;
}

impl TryFrom<JourneyWasCreated> for Journey {
    type Error = PersistenceConversionError;

    fn try_from(value: JourneyWasCreated) -> Result<Self, Self::Error> {
        let time_created = UNIX_EPOCH + Duration::new(value.time_created, 0u32);

        Ok(Self::new(
            ID::new(Uuid::parse_str(&value.journey_id)?),
            time_created,
            Title::build(value.title)?,
        ))
    }
}

/// Public error type related to converting service
#[derive(Error, Debug, Clone)]
pub enum PersistenceConversionError {
    /// errors related to parsing of UUID from string
    #[error("ID cannot get parsed: {0}")]
    CouldNotBuildUUID(#[from] uuid::Error),

    /// errors related to title or body creation from input
    #[error("Title/body cannot get built: {0}")]
    TitleOrBodyCannotBeBuilt(#[from] DomainError),
}