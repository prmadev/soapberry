//! [`Created`] is an special event which starts a new `RedMaple` and should be the first event of each
//! `RedMaple`.

use std::time::SystemTime;

use redmaple::id::ID;

/// Creates a new instance of Story
///
/// * `id`: is of type ID.
/// * `redmaple_id`: is of type ID.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Created {
    id: ID,
    created: SystemTime,
    redmaple_id: ID,
}

impl Created {
    /// Creates a new [`Created`] event
    ///
    /// * `view_mode`: set the view mode for this `RedMaple` `ViewMode`
    /// * `redmaple_id`: set the id of the the parent redmaple
    #[must_use]
    pub const fn new(id: ID, created: SystemTime, redmaple_id: ID) -> Self {
        Self {
            id,
            created,
            redmaple_id,
        }
    }

    /// returns the id of event
    #[must_use]
    pub const fn id(&self) -> &ID {
        &self.id
    }

    /// returns the id of the parent redmaple
    #[must_use]
    pub const fn redmaple_id(&self) -> &ID {
        &self.redmaple_id
    }

    /// returns the creation time of event
    #[must_use]
    pub const fn created(&self) -> &SystemTime {
        &self.created
    }
}

#[cfg(test)]
mod tests {
    use crate::argument::maple_created::Created;
    use redmaple::id::ID;

    #[test]
    fn could_make_event() {
        let red_maple_id = ID::new(uuid::Uuid::new_v4());
        let new_event = Created::new(
            ID::new(uuid::Uuid::new_v4()),
            std::time::SystemTime::now(),
            red_maple_id.clone(),
        );

        assert_eq!(new_event.redmaple_id(), &red_maple_id);
        assert_eq!(
            new_event.id().inner().to_string().len(),
            red_maple_id.inner().to_string().len()
        );
    }
}
