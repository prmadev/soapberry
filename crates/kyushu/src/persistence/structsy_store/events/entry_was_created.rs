#![allow(missing_docs)]
//! implementations for persisting [`EntryWasCreated`] event

use std::time::{self, UNIX_EPOCH};

use redmaple::id::ID;
use structsy::{Order, StructsyTx};
use structsy_derive::{queries, Persistent};
use whirlybird::journey::{body::Body, title::Title};

use crate::domain::{
    self,
    messages::events::{entry_was_created::EntryRepo, EventRepo},
};

use super::{FromPersistenceConversionError, ToPersistenceConversionError};

/// Event Stored for when an event was created
#[derive(Persistent, Clone)]
pub struct EntryWasCreated {
    id: String,
    time_created: u64,
    entry_id: String,
    title: Option<String>,
    body: Option<String>,
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

    /// seraches for specific id
    #[must_use]
    fn search_by_event_id(self, entry_id: &str) -> Self;

    /// returns all of the items in the order of the time of their creation
    #[must_use]
    fn order_by_time_created(self, time_created: Order) -> Self;
}

/// conversion: persistent -> domain
impl TryFrom<EntryWasCreated> for domain::messages::events::entry_was_created::EntryWasCreated {
    type Error = FromPersistenceConversionError;

    fn try_from(value: EntryWasCreated) -> Result<Self, Self::Error> {
        let time_created = time::UNIX_EPOCH + time::Duration::from_secs(value.time_created);
        let event_id = ID::new(
            uuid::Uuid::parse_str(&value.id)
                .map_err(FromPersistenceConversionError::CouldNotBuildUUID)?,
        );

        let entry_id = ID::new(
            uuid::Uuid::parse_str(&value.entry_id)
                .map_err(FromPersistenceConversionError::CouldNotBuildUUID)?,
        );

        let title = {
            match value.title.map(Title::build) {
                Some(r) => Some(r.map_err(FromPersistenceConversionError::TitleCannotBeBuilt)?),
                None => None,
            }
        };

        let body = {
            match value.body.map(Body::build) {
                Some(r) => Some(r.map_err(FromPersistenceConversionError::BodyCannotBeBuilt)?),
                None => None,
            }
        };

        Ok(Self::new(event_id, time_created, entry_id, title, body))
    }
}

/// conversion: domain -> persistent
impl TryFrom<domain::messages::events::entry_was_created::EntryWasCreated> for EntryWasCreated {
    type Error = ToPersistenceConversionError;

    fn try_from(
        value: domain::messages::events::entry_was_created::EntryWasCreated,
    ) -> Result<Self, Self::Error> {
        let time_created = value
            .time_created()
            .duration_since(UNIX_EPOCH)
            .map_err(|_e| ToPersistenceConversionError::TimeCouldNotBeParsedIntoU8)?
            .as_secs();

        Ok(Self {
            time_created,
            entry_id: value.entry_id().clone().inner().to_string(),
            title: value.title().map(|t| t.inner().to_string()),
            body: value.body().map(|b| b.inner().to_string()),
            id: redmaple::id::IDGiver::id(&value).inner().to_string(),
        })
    }
}

/// An implementation of Repo for the [`domain::messages::events::entry_was_created::EntryWasCreated`] using [`structsy::Structsy`]
pub struct StructsyStore(structsy::Structsy);

impl StructsyStore {
    /// creates a new [`StructsyStore`]
    #[must_use]
    pub const fn new(db: structsy::Structsy) -> Self {
        Self(db)
    }
}

impl EventRepo for StructsyStore {
    type Item = domain::messages::events::entry_was_created::EntryWasCreated;

    type EventError = EventRepoError;

    fn events_matching_id(&self, id: &redmaple::id::ID) -> Result<Self::Item, Self::EventError> {
        let query = self
            .0
            .query::<EntryWasCreated>()
            .search_by_event_id(&id.inner().to_string());

        let item = query
            .into_iter()
            .next() // we only need the first item, and it only should be one item, otherwise, there is something wrong
            .ok_or(Self::EventError::NoItemWithThatIDWasFound(id.clone()))?
            .1;

        let event = domain::messages::events::entry_was_created::EntryWasCreated::try_from(item)?;
        Ok(event)
    }

    fn all_events(&self) -> Result<Vec<Self::Item>, Self::EventError> {
        let query = self
            .0
            .query::<EntryWasCreated>()
            .order_by_time_created(Order::Asc);

        let (items, errors): (Vec<_>, Vec<_>) = query
            .into_iter()
            .map(|(_, ent)| {
                domain::messages::events::entry_was_created::EntryWasCreated::try_from(ent)
            })
            .partition(Result::is_ok);

        // managing errors
        if !errors.is_empty() {
            return Err(EventRepoError::ConversionFromPersistenceFailedGroup(
                errors.into_iter().filter_map(Result::err).collect(),
            ));
        };

        Ok(items.into_iter().filter_map(Result::ok).collect())
    }

    fn append(&self, item: Self::Item) -> Result<(), Self::EventError> {
        let mut tx = self
            .0
            .begin()
            .map_err(|e| EventRepoError::TXBeginFailed(e.to_string()))?;
        let ent = EntryWasCreated::try_from(item)?;
        tx.insert(&ent)
            .map_err(|e| EventRepoError::InsertionIntoDBFailed(e.to_string()))?;
        tx.commit()
            .map_err(|e| EventRepoError::CommitTXFailed(e.to_string()))?;

        Ok(())
    }
}

/// errors that happen when communicating with this event repo  
#[derive(Debug, thiserror::Error, Clone)]
pub enum EventRepoError {
    /// No Item with That ID could be found
    #[error("No Item with That ID could be found")]
    NoItemWithThatIDWasFound(ID),

    /// Could not convert into persistence layer
    #[error("Could not convert into persistence layer")]
    ConversionIntoPersistenceFailed(#[from] ToPersistenceConversionError),

    /// Could not convert back from persistence layer
    #[error("Could not convert back from persistence layer")]
    ConversionFromPersistenceFailed(#[from] FromPersistenceConversionError),

    #[error("Could not convert back from persistence layer {0:?}")]
    ConversionFromPersistenceFailedGroup(Vec<FromPersistenceConversionError>),

    #[error("Could begin transaction: {0}")]
    TXBeginFailed(String),

    #[error("Could insert item into database: {0}")]
    InsertionIntoDBFailed(String),

    #[error("Could not commit transaction: {0}")]
    CommitTXFailed(String),
}
impl EntryRepo for StructsyStore {
    type EntryError = EntryRepoError;

    fn events_matching_entry_id(&self, id: &ID) -> Result<Vec<Self::Item>, Self::EntryError> {
        let (items, errors): (Vec<_>, Vec<_>) = self
            .0
            .query::<EntryWasCreated>()
            .search_by_entry_id(&id.inner().to_string())
            .into_iter()
            .map(|(_, ent)| {
                domain::messages::events::entry_was_created::EntryWasCreated::try_from(ent)
            })
            .partition(Result::is_ok);

        // managing errors
        if !errors.is_empty() {
            return Err(EntryRepoError::ConversionFromPersistenceFailedGroup(
                errors.into_iter().filter_map(Result::err).collect(),
            ));
        };

        Ok(items
            .into_iter()
            .filter_map(Result::ok)
            .collect::<Vec<domain::messages::events::entry_was_created::EntryWasCreated>>())
    }
}

/// errors that happen when communicating with this event repo  
#[derive(Debug, thiserror::Error, Clone)]
pub enum EntryRepoError {
    #[error("Could not convert back from persistence layer {0:?}")]
    ConversionFromPersistenceFailedGroup(Vec<FromPersistenceConversionError>),
}
