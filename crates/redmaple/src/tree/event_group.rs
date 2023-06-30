//! [`EventGroup`] is the centeral way to plug-in your events and their logic
//!
//! To make event group this small I actually spent a full day working with different solutions and
//! trying different ways and pattern for implementing it
//! so enjoy

use std::error::Error;

use crate::id::Unique;

/// [`EventGroup`] trait describes the behavior of an event.
/// Specific implementaiton is not defined here
pub trait EventKind: Unique {
    /// Errors related to [`EventKind`]
    type EventKindError: Error;

    /// returns the time of the time that that event happened at
    #[must_use]
    fn time(&self) -> &time::OffsetDateTime;
}
