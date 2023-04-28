#![allow(missing_docs)]
//! implementations for persisting [`EntryWasCreated`] event

use std::time::UNIX_EPOCH;

use redmaple::{event_group::EventGroup, id::IDGiver};
use structsy_derive::{queries, Persistent};
use whirlybird::journey::JournalEvent;

use super::PersistenceConversionError;

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
pub trait Query {
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
