use std::time::SystemTime;

use redmaple::{id::ID, RedMaple};

use super::{post::Post, views::Views, Argument};

/// Sets a Content as published
#[derive(Debug, Clone)]
pub struct PostDeleted {
    id: ID,
    created: SystemTime,
    redmaple_id: ID,
    post_id: ID,
}

impl PostDeleted {
    /// Creates an event that states that some content has been deleted (invisible) to users.
    pub fn new(red_maple: &RedMaple<Argument, Views>, post: &Post<String, String>) -> Self {
        Self {
            id: ID::new(),
            created: std::time::SystemTime::now(),
            redmaple_id: red_maple.id().clone(),
            post_id: post.id().clone(),
        }
    }

    /// Gets the ID of the entity
    pub const fn id(&self) -> &ID {
        &self.id
    }

    /// Gets the ID of the redmaple that holds this event
    pub const fn redmaple_id(&self) -> &ID {
        &self.redmaple_id
    }

    /// Gets the inner content ID that this event is effecting on
    pub const fn post_id(&self) -> &ID {
        &self.post_id
    }

    /// returns the creation time of event
    #[must_use]
    pub const fn created(&self) -> &SystemTime {
        &self.created
    }
}
