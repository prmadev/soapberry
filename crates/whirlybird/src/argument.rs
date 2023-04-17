//! This is an implementation of the `EventGroup` used as an exmaple of the basic logic neccessary,
//! to create your own event type

use std::time::SystemTime;

use redmaple::{event_group::EventGroup, id::ID};
use thiserror::Error;

use self::{
    maple_created::Created,
    post::{Mode, Post},
};

mod maple_created;
mod post;
mod post_added;
mod post_deleted;
mod post_moded;
mod post_published;

/// Event hold all the events that could happened to a `RedMaple`
#[derive(Debug, Clone)]
pub enum Argument {
    /// States that a RedMaple is created
    Created(maple_created::Created),
    /// When a content is added. It does not neccessarily means that it is published
    PostAdded(post_added::PostCreated),
    /// Happens When a content is visible by all those that can view the RedMaple
    PostPublished(post_published::ContentPublished),
    /// Happens when the view mod of the post changes
    PostModed(post_moded::ContentModed),
    /// Happens when the content is no longer visible
    PostDeleted(post_deleted::PostDeleted),
}

/// Dialog creates an instance of a person talking
///
/// * `id`: is of the type [`ID`] and should be the same type as a [`Post`] ID
/// * `posted`: of the type [`std::time::SystemTime`] and shows the time the dialog was created
/// * `published`: [`Option<std::time::SystemTime>`] if published the time of publication.
/// * `mode`: of the type [`Mode`]
/// * `content`:  content which is for now `String`. TODO: make enum which can hold a subscription
/// to a media as well
pub struct Dialog {
    id: ID,
    posted: SystemTime,
    published: Option<SystemTime>,
    mode: Mode,
    content: String,
}

impl Dialog {
    /// retuns the [`ID`] of the [`Dialog`]
    #[must_use]
    pub const fn id(&self) -> &ID {
        &self.id
    }

    /// returns the time the [`Dialog`] was originally created
    #[must_use]
    pub const fn posted(&self) -> SystemTime {
        self.posted
    }

    /// retuns the [`Dialog`]'s status with regards to its publication
    #[must_use]
    pub const fn published(&self) -> Option<SystemTime> {
        self.published
    }

    /// returns the [`Mode`] in which the [`Dialog`] is in
    #[must_use]
    pub const fn mode(&self) -> &Mode {
        &self.mode
    }

    /// returns the content of the the dialog
    #[must_use]
    pub fn content(&self) -> &str {
        self.content.as_ref()
    }
}

impl From<&Post<String, String>> for Dialog {
    fn from(value: &Post<String, String>) -> Self {
        Self {
            id: value.id().clone(),
            posted: *value.date(),
            published: None,
            mode: Mode::HeadPost,
            content: value.content().clone(),
        }
    }
}

#[derive(Error, Debug)]
/// An enum which shows the errors reated to applying the event group
pub enum StateChangeError {
    /// States that the [`Dialog`] we are trying to publish is already at the state that was
    /// requested
    #[error("the dialog is already that way")]
    AlreadyDone,

    /// States that the target [`Dialog`] is not found in the ArgumentPosts
    #[error("there is no dialog with that Id")]
    NotFound,
}

impl EventGroup for Argument {
    type EventGroupError = StateChangeError;

    fn id(&self) -> &ID {
        match *self {
            Self::Created(ref e) => e.id(),
            Self::PostAdded(ref e) => e.id(),
            Self::PostPublished(ref e) => e.id(),
            Self::PostModed(ref e) => e.id(),
            Self::PostDeleted(ref e) => e.id(),
        }
    }

    fn redmaple_id(&self) -> &ID {
        match *self {
            Self::Created(ref e) => e.redmaple_id(),
            Self::PostAdded(ref e) => e.redmaple_id().inner(),
            Self::PostPublished(ref e) => e.redmaple_id().inner(),
            Self::PostModed(ref e) => e.redmaple_id().inner(),
            Self::PostDeleted(ref e) => e.redmaple_id().inner(),
        }
    }

    fn time(&self) -> &std::time::SystemTime {
        match *self {
            Self::Created(ref e) => e.created(),
            Self::PostAdded(ref e) => e.created(),
            Self::PostPublished(ref e) => e.created(),
            Self::PostModed(ref e) => e.created(),
            Self::PostDeleted(ref e) => e.created(),
        }
    }
}

impl Argument {
    /// Creates a new instance of `Argument::Created`
    #[must_use]
    pub const fn new_create_event(id: ID, created: SystemTime, redmaple_id: ID) -> Self {
        Self::Created(Created::new(id, created, redmaple_id))
    }
}
