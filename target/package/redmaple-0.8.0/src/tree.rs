//! redmaple is the central data-structure that is underlying the whole crate
use self::{event_group::EventGroup, id::ID};
use crate::view_mode::ViewMode;
use std::fmt::Debug;

/// event module holds the types and functions that events could take and the operations that they
/// can do.
pub mod event_group;
/// id module holds the implementation of ID type
pub mod id;

/// versioned keeps the version of an stateful item
pub mod versioned;

/// `RedMaple` is essentially a series of related events that form a state
///
/// * `id`: of type ID
/// * `view_mode`: an enum that holds set view mode of an `RedMaple`
/// * `events`: a list of entities that happened in time series
#[derive(Debug, Clone)]
pub struct RedMaple<T: EventGroup + Sized + Clone, V: ViewMode + Sized + Clone> {
    id: ID,
    view_mode: V,
    events: Vec<T>,
    subscribers: Vec<ID>,
}

impl<T: EventGroup + Sized + Clone, V: ViewMode + Sized + Clone> RedMaple<T, V> {
    /// creates a new instance of [`RedMaple`]
    ///
    /// * `view_mode`: sets the view mode of the `RedMaple`
    #[must_use]
    pub const fn new(view_mode: V, id: ID) -> Self {
        Self {
            id,
            view_mode,
            events: vec![],
            subscribers: vec![],
        }
    }

    /// Gets the view mode of the `RedMaple`
    pub const fn view_mode(&self) -> &V {
        &self.view_mode
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
    pub fn subscribers(&self) -> &[ID] {
        self.subscribers.as_ref()
    }

    /// sets a list of subscribers. You should prefer `add_subscriber` and `remove_subscriber`
    /// whereever possible
    pub fn set_subsribers(&mut self, subscribers: Vec<ID>) {
        self.subscribers = subscribers;
    }

    /// adds a subscriber to the list of subscribers to this tree.
    ///
    /// * `subscriber`: of type `ID`
    ///
    /// # Errors
    ///
    /// * `SubscriberError::IsAlreadyInTheList` : means that the ID that you are trying to add is
    /// alread in the list of subscribers. having two subscriptions to the same ID may mean twice
    /// the change messages which may mean data corruption
    pub fn add_subscriber(&mut self, subscriber: ID) -> Result<(), SubscriberError> {
        if self.subscribers.contains(&subscriber) {
            return Err(SubscriberError::IsAlreadyInTheList(subscriber));
        };

        self.subscribers.push(subscriber);

        Ok(())
    }

    /// adds a subscriber to the list of subscribers to this tree.
    ///
    /// * `subscriber`: of type `ID`
    ///
    /// # Errors
    ///
    /// * `SubscriberError::CouldNotFindSubscriber`: means that the id that is requested by you
    ///  to be removed is not found in the list of subscribers of this item.
    ///
    pub fn remove_subscriber(&mut self, subscriber: &ID) -> Result<(), SubscriberError> {
        match self.subscribers.iter().position(|x| x == subscriber) {
            Some(position) => {
                self.subscribers.swap_remove(position);
                Ok(())
            }

            None => Err(SubscriberError::CouldNotFindSubscriber(subscriber.clone())),
        }
    }
}

/// Error type when a dealing with a subscriber
#[derive(thiserror::Error, Debug)]
pub enum SubscriberError {
    /// when subscriber is in the list
    #[error("Could not find the subscriber you are looking for: {0}")]
    CouldNotFindSubscriber(ID),

    /// when subscriber is already in the list
    #[error("ID is already in the subscribers list: {0}")]
    IsAlreadyInTheList(ID),
}
