use std::time::SystemTime;

use redmaple::{id::ID, RedMaple};

use super::{post::Post, views::Views, Argument};

/// Sets a Content as published
#[derive(Debug, Clone)]
pub struct ContentPublished {
    id: ID,
    created: SystemTime,
    redmaple_id: ID,
    post_id: ID,
}

impl ContentPublished {
    pub fn new(red_maple: &RedMaple<Argument, Views>, post: &Post<String, String>) -> Self {
        Self {
            id: ID::new(),
            created: std::time::SystemTime::now(),
            redmaple_id: red_maple.id().clone(),
            post_id: post.id().clone(),
        }
    }

    pub const fn redmaple_id(&self) -> &ID {
        &self.redmaple_id
    }

    pub const fn post_id(&self) -> &ID {
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
