//! event definition and traits
use std::{collections::HashMap, error, fmt::Display};

use crate::{
    event_group::EventKind,
    id::{Unique, ID},
    RedMaple,
};

/// Seeks and finds every redmaple
pub trait SeekingElf: Send + Sync {
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
}

/// Tracker elf maps every maple tree  
pub trait TrackerElf: Send + Sync {
    /// Item should be able to return ID
    type Item: EventKind + Unique + Clone + Eq + PartialOrd + Ord;

    /// Errors that may happen in these functions
    type EventError: Display + error::Error + Send + Sync;

    /// Returns all the redmaples as a map
    ///
    /// # Errors
    ///
    /// if at any point it could not convert any item it will return error
    fn maples(&self) -> Result<Vec<&RedMaple<Self::Item>>, Self::EventError>;
}

/// Cartographer elf maps every maple tree  
pub trait CartographerElf: Send + Sync {
    /// Item should be able to return ID
    type Item: EventKind + Unique + Clone + Eq + PartialOrd + Ord;

    /// Errors that may happen in these functions
    type EventError: Display + error::Error + Send + Sync;

    /// Returns all the redmaples as a map
    ///
    /// # Errors
    ///
    /// if at any point it could not convert any item it will return error
    fn all_redmaples_as_map(&self) -> Result<HashMap<ID, &RedMaple<Self::Item>>, Self::EventError>;
}

/// BeeElf plants new maple tree  
pub trait BeeElf: Send + Sync {
    /// Item should be able to return ID
    type Item: EventKind + Unique + Clone + Eq + PartialOrd + Ord;

    /// Errors that may happen in these functions
    type EventError: Display + error::Error + Send + Sync;

    /// Appends an item to the database
    ///
    /// # Errors
    ///
    /// if it cannot convert types correctly, or it fails at any of the steps of adding item to the database,
    /// it will return an error
    fn plant(&self, item: RedMaple<Self::Item>) -> Result<(), Self::EventError>;
}

/// GardnerElf waters and tends to maple trees  
pub trait GardnerElf: Send + Sync {
    /// Item should be able to return ID
    type Item: EventKind + Unique + Clone + Eq + PartialOrd + Ord;

    /// Errors that may happen in these functions
    type EventError: Display + error::Error + Send + Sync;

    /// updates the redmaple
    ///
    /// # Errors
    ///
    /// if it cannot convert types correctly, or it fails at any of the steps of adding item to the database,
    /// it will return an error
    fn tend(&self, item: RedMaple<Self::Item>) -> Result<(), Self::EventError>;
}

/// defines the bare minimum implementation for storing events
pub trait FrostElf: Send + Sync + SeekingElf + CartographerElf + BeeElf + GardnerElf {}
