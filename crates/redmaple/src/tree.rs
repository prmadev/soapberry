//! redmaple is the central data-structure that is underlying the whole crate
use self::{event_group::EventGroup, id::ID};
use std::fmt::Debug;

/// event module holds the types and functions that events could take and the operations that they
/// can do.
pub mod event_group;
/// id module holds the implementation of ID type
pub mod id;

/// versioned keeps the version of an stateful item

/// `RedMaple` is essentially a series of related events that form a state
///
/// * `id`: of type ID
/// * `view_mode`: an enum that holds set view mode of an `RedMaple`
/// * `events`: a list of entities that happened in time series
#[derive(Debug, Clone)]
pub struct RedMaple<T: EventGroup + Sized + Clone> {
    id: ID,
    events: Vec<T>,
    subscribers: SubscriberList,
}

impl<T: EventGroup + Sized + Clone> RedMaple<T> {
    /// creates a new instance of [`RedMaple`]
    ///
    /// * `view_mode`: sets the view mode of the `RedMaple`
    #[must_use]
    pub const fn new(id: ID, events: Vec<T>, subscribers: SubscriberList) -> Self {
        Self {
            id,
            events,
            subscribers,
        }
    }

    /// Gets the ID of the `RedMaple`
    #[must_use]
    pub const fn id(&self) -> &ID {
        &self.id
    }

    /// Gets an array of the events of the `RedMaple`
    #[must_use]
    pub const fn events(&self) -> &Vec<T> {
        &self.events
    }

    /// Gets a list of subscribers (the subscribers that are listening to any changes happening to
    /// this item)
    #[must_use]
    pub const fn subscribers(&self) -> &SubscriberList {
        &self.subscribers
    }
}

/// Error type when a dealing with a subscriber
#[derive(thiserror::Error, Debug)]
pub enum SubscriberError {
    /// when subscriber is in the list
    #[error("Could not find the subscriber you are looking for: {0}")]
    CouldNotFindSubscriber(ID),
}

/// [`SubscriberList`] is wrapper around  `Vec<ID>` which is there to ensure that the subscriber
/// list follows some gurantees, like not having duplicates and being ordered.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SubscriberList(Vec<ID>);
impl SubscriberList {
    /// Creates [`SubscriberLists`] but first sorts the given [`ID`] list and and then checks for
    /// duplicated subscribers, if found removes duplicates.
    #[must_use]
    pub fn new(mut members: Vec<ID>) -> Self {
        members.sort();
        members.dedup();
        Self(members)
    }

    /// Creates a reference to see the inner vector.
    #[must_use]
    pub const fn inner(&self) -> &Vec<ID> {
        &self.0
    }

    /// Returns the inner vector and consumes itself in the process.
    #[must_use]
    #[allow(clippy::missing_const_for_fn)] // currently a destructor method cannot be const
    pub fn into_inner(self) -> Vec<ID> {
        self.0
    }
}
