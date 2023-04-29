//! representations of past effectful changes

use redmaple::id::ID;
pub mod entry_was_created;
pub mod journey_was_created;

/// An ID that comes from an event
#[derive(Debug, Clone)]
pub struct ValidEventID(ID);
impl ValidEventID {
    /// returns the contained ID
    #[must_use]
    pub const fn inner(&self) -> &ID {
        &self.0
    }

    /// returns the contained ID
    #[must_use]
    #[allow(clippy::missing_const_for_fn)]
    pub fn into_inner(self) -> ID {
        self.0
    }
}
