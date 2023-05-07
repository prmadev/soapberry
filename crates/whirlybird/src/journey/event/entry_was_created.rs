//! holds the information about an event of an information being created

use std::{fmt::Display, time::SystemTime};

use crate::journey::entity::{body::Body, title::Title};
use redmaple::{
    id::{IDGiver, ID},
    EventRepo,
};

use crate::journey::ValidEventID;

/// indicates that an [`Entry`] was created
pub struct EntryWasCreated {
    event_id: ValidEventID,
    time_created: SystemTime,
    entry_id: ID,
    title: Option<Title>,
    body: Option<Body>,
}

/// repo actions that are tied to entries
pub trait EntryRepo: EventRepo {
    /// Errors that may happen in these functions
    type EntryError: Display + std::error::Error;

    /// find every event of this type that matches this [`ID`]
    ///
    /// # Errors
    ///
    /// If at any point it fails to convert types correctly or fails to match any item it will return an error
    fn events_matching_entry_id(&self, id: &ID) -> Result<Vec<Self::Item>, Self::EntryError>;
}

/// traits for repos that hold [`domain::messages::events::entry_was_created::EntryWasCreated`]
pub trait Repo: EntryRepo + EventRepo + Send + Sync {
    /// Error that holds the entire stack of errors
    type EntryWasCreatedError: Display + From<Self::EntryError> + From<Self::EventError>;
}

impl EntryWasCreated {
    /// creates a new instance of [`EntryWasCreated`]
    #[must_use]
    pub const fn new(
        event_id: ID,
        time_created: SystemTime,
        entry_id: ID,
        title: Option<Title>,
        body: Option<Body>,
    ) -> Self {
        Self {
            event_id: ValidEventID(event_id),
            time_created,
            entry_id,
            title,
            body,
        }
    }

    /// returns the time that the event was created
    #[must_use]
    pub const fn time_created(&self) -> SystemTime {
        self.time_created
    }

    /// returns the [`ID`] that is created to ["entry"]
    #[must_use]
    pub const fn entry_id(&self) -> &ID {
        &self.entry_id
    }

    /// returns the title of the entry
    #[must_use]
    pub const fn title(&self) -> Option<&Title> {
        self.title.as_ref()
    }

    /// returns the title of the entry
    #[must_use]
    pub const fn body(&self) -> Option<&Body> {
        self.body.as_ref()
    }
}

impl IDGiver for EntryWasCreated {
    type Valid = ValidEventID;

    #[must_use]
    fn id(&self) -> &Self::Valid {
        &self.event_id
    }

    #[must_use]
    fn into_id(self) -> Self::Valid {
        self.event_id
    }
}
