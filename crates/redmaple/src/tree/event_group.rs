//! [`EventGroup`] is the centeral way to plug-in your events and their logic
//!
//! To make event group this small I actually spent a full day working with different solutions and
//! trying different ways and pattern for implementing it
//! so enjoy

use std::{error::Error, time::SystemTime};

use super::{id::ID, versioned::Versioned};

/// [`EventGroup`] trait describes the behavior of an event.
/// Specific implementaiton is not defined here
///
/// # Example
///
/// ```
///    use redmaple::event_group::EventGroup;
///    use std::time::SystemTime;
///    use thiserror::Error;
///    use redmaple::tree::{id::ID, versioned::Versioned};
///
///
///    struct State(String, u64);
///
///    impl Versioned for State {
///         fn version(&self) -> u64 {
///            self.1
///         }
///         fn increment_version(&mut self) {
///            self.1 += 1;
///         }
///    }
///
///    struct Eg(ID, ID, std::time::SystemTime, String);
///
///    #[derive(Error, Debug)]
///    enum EventGroupErrorLocal {
///    }
///
///    impl EventGroup for Eg {
///        type State  = State;
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
///
///        fn has_the_same_contents(&self, other: &Self) -> bool{
///            self.3 == other.3
///        }
///
///        fn apply_to(&self, state: &mut Self::State) -> Result<(), Self::EventGroupError> {
///            state.0 = self.3.clone();
///            state.increment_version();
///            Ok(())
///        }
///    }
///
///    let ev1 = Eg(ID::new(), ID::new(), SystemTime::now(), String::from("first_text"));
///    let ev2 = Eg(ID::new(), ID::new(), SystemTime::now(), String::from("second_text"));
///
///
///    // the two instances should not have the same [`ID`]
///    assert_ne!(ev1.id(), ev2.id());
///
///    // however both have the same content: `String::from("")`
///    assert!(!ev1.has_the_same_contents(&ev2));
///    let mut state_instance = State(String::from(""), 0);
///
///    ev1.apply_to(&mut state_instance);
///    assert_eq!(state_instance.version(), 1);
///    assert_eq!(state_instance.0, "first_text");
///
///    ev2.apply_to(&mut state_instance);
///    assert_eq!(state_instance.version(), 2);
///    assert_eq!(state_instance.0, "second_text");
/// ```
pub trait EventGroup {
    /// State is the a type of object that shows the state of the [`EventGroup`]
    type State: Versioned;

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

    /// checks if the event have the same content of another event, but does not check for date
    /// and id which are probably unique to each event
    fn has_the_same_contents(&self, other: &Self) -> bool;

    /// applys the side-effects to the State.
    ///
    /// # Errors
    ///
    /// * [`EventGroupError`]: shows any error that is related to applying the event. This should
    /// not contain any domain logic here. domain logic errors should only be present during the
    /// commands of the redmaple.
    fn apply_to(&self, state: &mut Self::State) -> Result<(), Self::EventGroupError>;
}
