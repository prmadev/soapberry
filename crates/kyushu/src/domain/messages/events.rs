//! representations of past effectful changes

use std::fmt::Display;

use redmaple::id::{IDGiver, ID};
pub mod entry_was_created;

/// An ID that comes from an event
#[derive(Debug, Clone)]
pub struct ValidEventID(ID);
impl ValidEventID {
    /// returns the contained ID
    #[must_use]
    pub const fn inner(&self) -> &ID {
        &self.0
    }

    /// returns the contained ID
    #[must_use]
    #[allow(clippy::missing_const_for_fn)]
    pub fn into_inner(self) -> ID {
        self.0
    }
}

/// defines the bare minimum implementation for storing events
pub trait EventRepo {
    /// Item should be able to return ID
    type Item: IDGiver;

    /// Errors that may happen in these functions
    type EventError: Display + std::error::Error;

    /// returns events by finding the first one that matches the item
    ///
    /// # Errors
    ///
    /// if it cannot find any item or could not convert any of the types it will return an error.
    fn events_matching_id(&self, id: &ID) -> Result<Self::Item, Self::EventError>;

    /// returns all the events
    ///
    /// # Errors
    ///
    /// if at any point it could not convert any item it will return error
    fn all_events(&self) -> Result<Vec<Self::Item>, Self::EventError>;

    /// appends an item to the database
    ///
    /// # Errors
    ///
    /// if it cannot convert types correctly, or it fails at any of the steps of adding item to the database,
    /// it will return an error
    fn append(&self, item: Self::Item) -> Result<(), Self::EventError>;
}
