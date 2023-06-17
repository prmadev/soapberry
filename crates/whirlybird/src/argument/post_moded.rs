use std::time::SystemTime;

use redmaple::{
    id::{IDGiver, ID},
    RedMaple, ValidRedMapleID,
};

use super::{
    post::{Mode, ValidPostID},
    Argument,
};

/// Changes the mode of a content
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContentModed {
    id: ID,
    created: SystemTime,
    redmaple_id: ValidRedMapleID,
    post_id: ValidPostID,
    new_mod: Mode,
}

impl ContentModed {
    /// Creates an event that states that some content has changed their mod to a given one.
    pub fn new(
        id: ID,
        created: SystemTime,
        red_maple: &RedMaple<Argument>,
        post_id: ValidPostID,
        new_mod: Mode,
    ) -> Self {
        Self {
            id,
            created,
            redmaple_id: red_maple.id().clone(),
            post_id,
            new_mod,
        }
    }

    /// Gets the ID of the entity
    pub const fn id(&self) -> &ID {
        &self.id
    }

    /// Gets the ID of the redmaple that holds this event
    pub const fn redmaple_id(&self) -> &ValidRedMapleID {
        &self.redmaple_id
    }

    /// Gets the inner content ID that this event is effecting on
    pub const fn post_id(&self) -> &ValidPostID {
        &self.post_id
    }

    /// return the new mode that this event makes
    pub const fn new_mod(&self) -> &Mode {
        &self.new_mod
    }

    /// returns the creation time of event
    #[must_use]
    pub const fn created(&self) -> &SystemTime {
        &self.created
    }
}
