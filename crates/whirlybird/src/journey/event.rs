//! Welcome to the domain of events, where the essence of experience resides.

use redmaple::id::{ValidID, ID};

/// Behold the ValidEventID, a sacred conduit that safeguards the harmony of associations with JournalEvents.
///
/// This struct, adorned with its Debug, Clone, PartialEq, Eq, Deserialize, and Serialize powers,
/// envelops an ID bestowed upon us by the redmaple realm.
///
/// As we traverse the realm of ValidID, we encounter two sacred rites:
/// - `inner()`: Gaze upon the inner sanctum of the JournalEvent's ID, as revealed by this function.
///   Feel its mighty essence flowing through your mortal fingertips.
/// - `into_inner()`: Transcendence awaits those who dare to partake in this sacrament.
///   Surrender the ValidEventID and claim the untamed power of the JournalEvent's ID.
///
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ValidEventID(pub(super) ID);

impl ValidID for ValidEventID {
    /// Witness the revelation of the JournalEvent's inner sanctum, the revered ID.
    ///
    /// By beholding its sacred presence, we may bask in the profound connection with the JournalEvent,
    /// gaining insight into its true nature and purpose.
    #[must_use]
    fn inner(&self) -> &ID {
        &self.0
    }

    /// Dare to embark on a transformative journey, shedding the cloak of ValidEventID,
    /// and embracing the unbound power of the JournalEvent's ID.
    ///
    /// As you assume this new form, be prepared to face the responsibilities and consequences
    /// of wielding such raw potential.
    #[must_use]
    fn into_inner(self) -> ID {
        self.0
    }
}
