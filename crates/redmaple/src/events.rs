//! event definition and traits
use std::{collections::HashMap, error, fmt::Display};

use crate::{
    event_group::EventKind,
    id::{Unique, ID},
    RedMaple,
};

/// defines the bare minimum implementation for storing events
pub trait FrostElf: Send + Sync {
    /// Item should be able to return ID
    type Item: EventKind + Unique + Clone + Eq + PartialOrd + Ord;

    /// Errors that may happen in these functions
    type EventError: Display + error::Error + Send + Sync;

    /// Returns events by finding the first one that matches the item
    ///
    /// # Errors
    ///
    /// if it cannot find any item or could not convert any of the types it will return an error.
    fn redmaple_matching_id(&self, id: &ID) -> Result<&RedMaple<Self::Item>, Self::EventError>;

    /// Returns events by finding the first one that is similar to the item
    ///
    /// # Errors
    ///
    /// if it cannot find any item or could not convert any of the types it will return an error.
    fn redmaple_similar_id(&self, id: &ID) -> Result<&RedMaple<Self::Item>, Self::EventError>;

    /// Returns all the events
    ///
    /// # Errors
    ///
    /// if at any point it could not convert any item it will return error
    fn all_events(&self) -> Result<&HashMap<ID, RedMaple<Self::Item>>, Self::EventError>;

    /// Appends an item to the database
    ///
    /// # Errors
    ///
    /// if it cannot convert types correctly, or it fails at any of the steps of adding item to the database,
    /// it will return an error
    fn save(&self, item: RedMaple<Self::Item>) -> Result<(), Self::EventError>;
}
