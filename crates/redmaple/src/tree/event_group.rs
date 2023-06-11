//! [`EventGroup`] is the centeral way to plug-in your events and their logic
//!
//! To make event group this small I actually spent a full day working with different solutions and
//! trying different ways and pattern for implementing it
//! so enjoy

use std::{error::Error, time::SystemTime};

use super::ID;

/// [`EventGroup`] trait describes the behavior of an event.
/// Specific implementaiton is not defined here
///
/// # Example
///
/// ```
///    use redmaple::event_group::EventGroup;
///    use std::time::SystemTime;
///    use thiserror::Error;
///    use redmaple::tree::{id::ID};
///
///
///
///    struct Eg(ID, ID, std::time::SystemTime, String);
///
///    #[derive(Error, Debug)]
///    enum EventGroupErrorLocal {
///    }
///
///    impl EventGroup for Eg {
///        type EventGroupError = EventGroupErrorLocal;
///
///        fn id(&self) -> &ID {
///            &self.0
///        }
///
///        fn redmaple_id(&self) -> &ID {
///            &self.1
///        }
///
///        fn time(&self) -> &SystemTime {
///            &self.2
///        }
///    }
///
///    let ev1 = Eg(ID::new(uuid::Uuid::new_v4()), ID::new(uuid::Uuid::new_v4()), SystemTime::now(), String::from("first_text"));
///    let ev2 = Eg(ID::new(uuid::Uuid::new_v4()), ID::new(uuid::Uuid::new_v4()), SystemTime::now(), String::from("second_text"));
///
///
///    // the two instances should not have the same [`ID`]
///    assert_ne!(ev1.id(), ev2.id());
/// ```
pub trait EventGroup {
    /// Errors related to [`EventGroup`]
    type EventGroupError: Error;

    /// returns the a reference to the inner [`ID`] of the event
    #[must_use]
    fn id(&self) -> &ID;

    /// returns the id of the parent [`RedMaple`]
    #[must_use]
    fn redmaple_id(&self) -> &ID;

    /// returns the time of the time that that event happened at
    #[must_use]
    fn time(&self) -> &SystemTime;
}
