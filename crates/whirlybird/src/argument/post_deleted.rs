use std::time::SystemTime;

use redmaple::{
    id::{IDGiver, ID},
    RedMaple, ValidRedMapleID,
};

use super::{post::ValidPostID, Argument};

/// Sets a Content as published
#[derive(Debug, Clone)]
pub struct PostDeleted {
    id: ID,
    created: SystemTime,
    redmaple_id: ValidRedMapleID,
    post_id: ValidPostID,
}

impl PostDeleted {
    /// Creates an event that states that some content has been deleted (invisible) to users.
    pub fn new(
        id: ID,
        created: SystemTime,
        red_maple: &RedMaple<Argument>,
        post_id: ValidPostID,
    ) -> Self {
        Self {
            id,
            created,
            redmaple_id: red_maple.id().clone(),
            post_id,
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

    /// returns the creation time of event
    #[must_use]
    pub const fn created(&self) -> &SystemTime {
        &self.created
    }
}
