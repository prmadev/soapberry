//! events reside in here

use redmaple::id::{ValidID, ID};

/// A thin wrapper around [`ID`] that validates that the [`ID`] is coming from an [`JournalEvent`]
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ValidEventID(pub(super) ID);

impl ValidID for ValidEventID {
    /// exposes the inner [`ID`] of the [`JournalEvent`]
    #[must_use]
    fn inner(&self) -> &ID {
        &self.0
    }

    /// exposes the inner [`ID`] of the [`JournalEvent`]
    #[must_use]
    fn into_id(self) -> ID {
        self.0
    }
}
