use std::time::SystemTime;

use redmaple::{id::ID, RedMaple};

use super::{post::Post, views::Views, Argument};

/// Adds Content to that redmaple
#[derive(Debug, Clone)]
pub struct PostCreated {
    id: ID,
    created: SystemTime,
    redmaple_id: ID,
    post: Post<String, String>,
}

impl PostCreated {
    /// Creates an event that states that some content has been added to an existing `RedMaple`.
    pub fn new(red_maple: &RedMaple<Argument, Views>, post: Post<String, String>) -> Self {
        Self {
            id: ID::new(),
            created: std::time::SystemTime::now(),
            redmaple_id: red_maple.id().clone(),
            post,
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

    /// Gets the inner content that is represented by this event
    pub const fn content(&self) -> &Post<String, String> {
        &self.post
    }

    /// returns the creation time of event
    #[must_use]
    pub const fn created(&self) -> &SystemTime {
        &self.created
    }

    /// returns the post information
    #[must_use]
    pub const fn post(&self) -> &Post<String, String> {
        &self.post
    }
}
