//! these are the persistence <-> redmaple operators

use redmaple::{id::ID, RedMaple};
use whirlybird::journey::JournalEvent;

#[derive(Debug, Clone)]
/// Journal is an immutable redmaple
pub struct Journal(RedMaple<JournalEvent>);

impl Journal {
    /// creates a new journal from the given events
    #[must_use]
    pub const fn new(id: ID, events: Vec<JournalEvent>) -> Self {
        Self(RedMaple::<JournalEvent>::new(id, events))
    }
}
