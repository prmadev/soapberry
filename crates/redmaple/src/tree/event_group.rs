//! [`EventGroup`] is the centeral way to plug-in your events and their logic
//!
//! To make event group this small I actually spent a full day working with different solutions and
//! trying different ways and pattern for implementing it
//! so enjoy

use std::error::Error;

use crate::id::ID;

/// [`EventGroup`] trait describes the behavior of an event.
/// Specific implementaiton is not defined here
pub trait EventGroup {
    /// Errors related to [`EventGroup`]
    type EventGroupError: Error;

    /// returns the a reference to the inner [`ID`] of the event
    #[must_use]
    fn id(&self) -> &ID;

    /// returns the time of the time that that event happened at
    #[must_use]
    fn time(&self) -> &time::OffsetDateTime;
}
