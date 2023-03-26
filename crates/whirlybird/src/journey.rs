//! it hold the logic of a contemplating journal.
//! it is difference from a zettelkasten-style journaling.
//! in that each node entry has a time associated with it.
//! and forms named relation ships.
//! these relationships form journeys
pub mod body;
pub mod entry;
pub mod link;
pub mod title;

use std::time::SystemTime;

use getset_scoped::Getters;
use redmaple::id::ID;

use self::{
    body::Body,
    entry::{Entry, ValidEntryID},
    link::Link,
    title::Title,
};

/// Event hold all the events that could happened to a `RedMaple`
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Journal {
    /// Event: An [`Entry`] was created.
    EntryCreated(Entry),

    /// Event: An already existing [`Entry`] was **added** to an already existing [`Journey`].
    EntryAddedToJourney(ValidEntryID, ValidJourneyID),

    /// Event: An already existing [`Entry`] was **removed** from an already existing [`Journey`].
    EntryRemovedFromJourney(ValidEntryID, ValidJourneyID),

    /// Event: An already existing [`Entry`] was updated to a new version.
    EntryUpdated(ValidEntryID, Entry),

    /// Event: An already existing [`Entry`] added a new [`Link`].
    EntryLinked(Link),

    /// Event: A new [`Journey`] was created.
    JourneyCreated(Journey),

    /// Event: An already existing [`Journey`] Got a new [`Title`].
    JourneyRenamed(ValidJourneyID, Title),

    /// Event: An already existing [`Journey`] was deleted.
    JourneyDeleted(ValidJourneyID),
}

/// A thin wrapper around [`ID`] that validates that the [`ID`] is coming from an [`Journey`]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidJourneyID(ID);

impl ValidJourneyID {
    /// exposes the inner [`ID`] of the [`Journey`]
    #[must_use]
    pub const fn inner(&self) -> &ID {
        &self.0
    }
}

/// [`Journey`] is the holder of meta information for journeys
#[derive(Clone, Debug, Getters, PartialEq, Eq)]
pub struct Journey {
    /// The unique [`ID`] of certain [`Journey`].
    #[getset(get = "pub")]
    id: ValidJourneyID,

    /// The time it was created.
    #[getset(get = "pub")]
    time_created: SystemTime,

    /// [`Title`] of the [`Entry`]
    #[getset(get = "pub")]
    title: Title,
}

impl Journey {
    /// new creates a new instance of [`Journey`]
    #[must_use]
    pub const fn new(id: ID, time_created: SystemTime, title: Title) -> Self {
        Self {
            id: ValidJourneyID(id),
            time_created,
            title,
        }
    }
}

/// [`ObjectType`] specifies the type of object
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ObjectType {
    /// an object that is held in this [`Redmaple`]
    Internal,
    /// an object that is held in other [`Redmaple`]
    External,
    ///  n object that points to an specific time
    Time,
}

/// Errors that are resulted from functions  and emthods inside [`journey`]
#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub enum DomainError {
    /// For when a text field should contain 1 or more characters
    #[error("Text Cannot have 0 length")]
    TextCannotBeEmpty,
}
