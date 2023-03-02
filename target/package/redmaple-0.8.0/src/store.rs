//! store module holds the persistence and stateful storage abstractions to work with this crate
use crate::{
    tree::{event_group::EventGroup, id::ID, RedMaple},
    view_mode::ViewMode,
};

/// This trait is an adaptor trait for the storage that holds the ID stuff that this package uses in order to validate and operate.
pub trait EventStorage<H, V>
where
    H: EventGroup + Sized + Clone,
    V: ViewMode + Sized + Clone,
{
    /// This function takes an [`id`] and checks if the [`ID`] matches event with the same [`ID`].
    fn id_exists(&self, id: &ID) -> bool;

    /// Adds an event to the [`EventStore`].
    ///
    /// # Errors
    ///
    /// This function should return errors that shows the submited event could not consistenty
    /// add event to the persistent layer
    fn add_event(&mut self, event: H) -> Result<(), SaveError>;

    /// Retrieve all the events of a vector.
    fn get_events(&self) -> Option<Vec<H>>;

    /// Retrieve an specific event with the specified ID.
    ///
    /// # Errors
    ///
    /// This function should return errors that shows the for some reason event could not be
    /// accessed
    fn get_event(&self, id: &ID) -> Result<H, FindError>;

    /// Retrieve all the `RedMaples` in the database
    fn get_redmaples(&self) -> Option<Vec<RedMaple<H, V>>>;
}

/// Errors that could happened during adding of an event to the event store
#[derive(Debug, thiserror::Error)]
pub enum SaveError {
    // #[error("Could Not be found")]
    // NotFound,
}

/// Errors that could happened when looking for an specific event
#[derive(Debug, thiserror::Error)]
pub enum FindError {
    /// NotFound happens when the event store could not find an event matching the requested
    /// parameters.
    #[error("Could Not be found")]
    NotFound,
}

/// State store is the adaptor trait that holds the current state of the system.
pub trait StateStorage<H, V>
where
    H: EventGroup + Sized + Clone,
    V: ViewMode + Sized + Clone,
{
    /// Apply applies the event to the current state and creates a new state
    ///
    /// # Errors
    ///
    /// This function should return errors that shows the submited event could not consistenty
    /// change the state
    fn apply(&self, event: H) -> Result<(), ApplyError>;
    /// Gives the full list of stories
    fn get_stories(&self) -> Option<Vec<RedMaple<H, V>>>;
}

/// Errors that happened when applying an event to the `StateStore`
#[derive(Debug, thiserror::Error)]
pub enum ApplyError {
    // #[error("Could Not be found")]
    // NotFound,
}
