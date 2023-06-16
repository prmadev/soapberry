//! event definition and traits
use std::{collections::HashMap, fmt::Display};

use crate::{
    event_group::EventGroup,
    id::{IDGiver, ID},
    RedMaple,
};

/// defines the bare minimum implementation for storing events
pub trait EventRepo: Clone {
    /// Item should be able to return ID
    type Item: EventGroup + IDGiver + Clone;

    /// Errors that may happen in these functions
    type EventError: Display + std::error::Error;

    /// returns events by finding the first one that matches the item
    ///
    /// # Errors
    ///
    /// if it cannot find any item or could not convert any of the types it will return an error.
    fn redmaple_matching_id(&self, id: &ID) -> Result<&RedMaple<Self::Item>, Self::EventError>;

    /// returns all the events
    ///
    /// # Errors
    ///
    /// if at any point it could not convert any item it will return error
    fn all_events(&self) -> Result<&HashMap<ID, RedMaple<Self::Item>>, Self::EventError>;

    /// appends an item to the database
    ///
    /// # Errors
    ///
    /// if it cannot convert types correctly, or it fails at any of the steps of adding item to the database,
    /// it will return an error
    fn append(&self, item: Self::Item) -> Result<(), Self::EventError>;
}
