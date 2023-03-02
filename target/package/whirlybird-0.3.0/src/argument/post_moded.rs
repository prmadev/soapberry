use std::time::SystemTime;

use redmaple::{id::ID, RedMaple};

use super::{
    post::{Mode, Post},
    views::Views,
    Argument,
};

/// Changes the mode of a content
#[derive(Debug, Clone)]
pub struct ContentModed {
    id: ID,
    created: SystemTime,
    redmaple_id: ID,
    post_id: ID,
    new_mod: Mode,
}

impl ContentModed {
    /// Creates an event that states that some content has changed their mod to a given one.
    pub fn new(
        red_maple: &RedMaple<Argument, Views>,
        post: &Post<String, String>,
        new_mod: Mode,
    ) -> Self {
        Self {
            id: ID::new(),
            created: std::time::SystemTime::now(),
            redmaple_id: red_maple.id().clone(),
            post_id: post.id().clone(),
            new_mod,
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
