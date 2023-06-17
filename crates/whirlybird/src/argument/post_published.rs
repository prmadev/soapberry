use std::time::SystemTime;

use redmaple::{
    id::{IDGiver, ID},
    RedMaple, ValidRedMapleID,
};

use super::{post::ValidPostID, Argument};

/// Sets a Content as published
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContentPublished {
    id: ID,
    created: SystemTime,
    redmaple_id: ValidRedMapleID,
    post_id: ValidPostID,
}

impl ContentPublished {
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

    pub const fn redmaple_id(&self) -> &ValidRedMapleID {
        &self.redmaple_id
    }

    pub const fn post_id(&self) -> &ValidPostID {
        &self.post_id
    }

    pub const fn id(&self) -> &ID {
        &self.id
    }

    /// returns the creation time of event
    #[must_use]
    pub const fn created(&self) -> &SystemTime {
        &self.created
    }
}
