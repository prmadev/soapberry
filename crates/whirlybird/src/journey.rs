//! it hold the logic of a contemplating journal.
//! it is difference from a zettelkasten-style journaling.
//! in that each node entry has a time associated with it.
//! and forms named relation ships.
//! these relationships form journeys
pub mod entity;
pub use entity::*;

pub mod event;
pub use event::*;

use crate::journey::event::ValidEventID;
use std::time::SystemTime;

use redmaple::{
    event_group::EventGroup,
    id::{IDGiver, ID},
};

/// [`JournelaEvent`] holds the meta data for [`Journal`] event
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct JournalEvent {
    event_id: ValidEventID,
    time: SystemTime,
    data: Journal,
}

impl JournalEvent {
    /// this will create a new Journal event
    #[must_use]
    pub const fn new(event_id: ID, time: SystemTime, data: Journal) -> Self {
        Self {
            event_id: ValidEventID(event_id),
            time,
            data,
        }
    }

    /// returns the valid ID of the event
    #[must_use]
    pub const fn event_id(&self) -> &ValidEventID {
        &self.event_id
    }

    /// returns the specific data to be acted on
    #[must_use]
    pub const fn data(&self) -> &Journal {
        &self.data
    }
}

impl EventGroup for JournalEvent {
    type EventGroupError = DomainError;

    fn id(&self) -> &ID {
        self.event_id().inner()
    }

    fn time(&self) -> &SystemTime {
        &self.time
    }
}

impl IDGiver for JournalEvent {
    type Valid = event::ValidEventID;

    fn id(&self) -> &Self::Valid {
        &self.event_id
    }

    fn into_id(self) -> Self::Valid {
        self.event_id
    }
}

/// Event hold all the events that could happened to a `RedMaple`
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub enum Journal {
    /// Event: An [`Entry`] was created.
    EntryCreated(Entry),

    /// Event: An already existing [`Entry`] was updated to a new version.
    EntryUpdated(ValidEntryID, Entry),

    /// Event: A new [`Journey`] was created.
    JourneyCreated(Journey),

    /// Event: An already existing [`Journey`] was deleted.
    JourneyDeleted(ValidJourneyID),
}

/// A thin wrapper around [`ID`] that validates that the [`ID`] is coming from an [`Journey`]
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ValidJourneyID(ID);

impl ValidJourneyID {
    /// exposes the inner [`ID`] of the [`Journey`]
    #[must_use]
    pub const fn inner(&self) -> &ID {
        &self.0
    }
}

impl IDGiver for Journey {
    type Valid = ValidJourneyID;

    fn id(&self) -> &Self::Valid {
        &self.id
    }

    fn into_id(self) -> Self::Valid {
        self.id
    }
}

/// [`Journey`] is the holder of meta information for journeys
#[derive(Clone, Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct Journey {
    /// The unique [`ID`] of certain [`Journey`].
    id: ValidJourneyID,

    /// The time it was created.
    time_created: SystemTime,
}

impl Journey {
    /// new creates a new instance of [`Journey`]
    #[must_use]
    pub const fn new(id: ID, time_created: SystemTime) -> Self {
        Self {
            id: ValidJourneyID(id),
            time_created,
        }
    }
}

// /// [`ObjectType`] specifies the type of object
// #[derive(Clone, Debug, PartialEq, Eq)]
// pub enum ObjectType {
//     /// an object that is held in this [`Redmaple`]
//     Internal,
//     /// an object that is held in other [`Redmaple`]
//     External,
//     ///  n object that points to an specific time
//     Time,
// }

/// Errors that are resulted from functions  and emthods inside [`journey`]
#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub enum DomainError {
    /// For when a text field should contain 1 or more characters
    #[error("Text Cannot have 0 length")]
    TextCannotBeEmpty,
}
