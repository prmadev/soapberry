//! persisted events in the structsy store

#![allow(missing_docs)]
use std::time::{Duration, UNIX_EPOCH};

use redmaple::{
    event_group::EventGroup,
    id::{IDGiver, ID},
};
use structsy::derive::{queries, Persistent};
use thiserror::Error;
use uuid::Uuid;
use whirlybird::journey::{title::Title, DomainError, JournalEvent, Journey};

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

/// Event Stored for when an event was created
#[derive(Persistent, Clone)]
pub struct EntryWasCreated {
    time_created: u64,
    entry_id: String,
    title: Option<String>,
    body: Option<String>,
    links: Vec<String>,
    journeys: Vec<String>,
}

/// queries relatedo to the Object
#[queries(EntryWasCreated)]
pub trait EntryWasCreatedQuery {
    /// searches for matching titles
    #[must_use]
    fn search_title(self, title: &str) -> Self;
    /// seraches for specific id
    #[must_use]
    fn search_by_entry_id(self, entry_id: &str) -> Self;
}

impl TryFrom<JournalEvent> for EntryWasCreated {
    type Error = PersistenceConversionError;

    fn try_from(value: JournalEvent) -> Result<Self, Self::Error> {
        let data = match value.data() {
            whirlybird::journey::Journal::EntryCreated(entry_data) => Ok(entry_data),
            _ => Err(PersistenceConversionError::IncorrectEventType),
        }?;

        // let time_created = UNIX_EPOCH + Duration::new(value.time(), 0u32);

        let time_created = value
            .time()
            .duration_since(UNIX_EPOCH)
            .map_err(|_e| PersistenceConversionError::TimeCouldNotBeParsedIntoU8)?
            .as_secs();

        Ok(Self {
            time_created,
            entry_id: data.id().clone().inner().to_string(),
            title: data.title().clone().map(|t| t.inner().to_string()),
            body: data.body().clone().map(|b| b.inner().to_string()),
            links: data
                .links()
                .iter()
                .map(|l| l.id().inner().to_string())
                .collect(),
            journeys: data
                .journeys()
                .iter()
                .map(|j| j.inner().inner().to_string())
                .collect(),
        })
    }
}
