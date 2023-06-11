//! events reside in here

use redmaple::id::ID;
pub mod entry_was_created;

/// A thin wrapper around [`ID`] that validates that the [`ID`] is coming from an [`JournalEvent`]
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ValidEventID(pub(super) ID);

impl ValidEventID {
    /// exposes the inner [`ID`] of the [`JournalEvent`]
    #[must_use]
    pub const fn inner(&self) -> &ID {
        &self.0
    }
}
