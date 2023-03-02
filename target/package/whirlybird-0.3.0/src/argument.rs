//! This is an implementation of the `EventGroup` used as an exmaple of the basic logic neccessary,
//! to create your own event type

use std::time::SystemTime;

use redmaple::{event_group::EventGroup, id::ID, versioned::Versioned};
use thiserror::Error;

use self::{
    maple_created::Created,
    post::{Mode, Post},
    views::{BlogMode, Views},
};

mod maple_created;
mod post;
mod post_added;
mod post_deleted;
mod post_moded;
mod post_published;
pub mod views;

/// Event hold all the events that could happened to a `RedMaple`
#[derive(Debug, Clone)]
pub enum Argument {
    /// States that a RedMaple is created
    Created(maple_created::Created<views::Views>),
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

/// holds the state of an [`Argument`]
pub struct State {
    version: u64,
    view: Views,
    dialogs: Vec<Dialog>,
}

impl State {
    /// Returns a reference to the view of this [`ArgumentState`].
    pub const fn view(&self) -> &Views {
        &self.view
    }

    /// Sets the view of this [`ArgumentState`].
    fn set_view(&mut self, view: Views) {
        self.view = view;
    }

    /// Returns a reference to the posts of this [`ArgumentState`].
    #[must_use]
    pub fn dialogs(&self) -> &[Dialog] {
        self.dialogs.as_ref()
    }

    /// Adds a [`Dialog`]
    fn add_dialog(&mut self, post: Dialog) {
        self.dialogs.push(post);
    }

    fn publish_dialog(&mut self, id: &ID, time: SystemTime) -> Result<(), StateChangeError> {
        match self.dialogs.iter_mut().find(|dialog| dialog.id() == id) {
            Some(mut dialog) => {
                if dialog.published().is_some() {
                    Err(StateChangeError::AlreadyDone)
                } else {
                    dialog.published = Some(time);
                    Ok(())
                }
            }
            None => Err(StateChangeError::NotFound),
        }
    }

    fn unpublish_dialog(&mut self, id: &ID) -> Result<(), StateChangeError> {
        match self.dialogs.iter_mut().find(|dialog| dialog.id() == id) {
            Some(mut dialog) => {
                if dialog.published().is_none() {
                    Err(StateChangeError::AlreadyDone)
                } else {
                    dialog.published = None;
                    Ok(())
                }
            }
            None => Err(StateChangeError::NotFound),
        }
    }

    fn mode_dialog(&mut self, id: &ID, mode: Mode) -> Result<(), StateChangeError> {
        match self.dialogs.iter_mut().find(|dialog| dialog.id() == id) {
            Some(mut dialog) => {
                if dialog.mode() == &mode {
                    Err(StateChangeError::AlreadyDone)
                } else {
                    dialog.mode = mode;
                    Ok(())
                }
            }
            None => Err(StateChangeError::NotFound),
        }
    }
}

impl Versioned for State {
    fn version(&self) -> u64 {
        self.version
    }

    fn increment_version(&mut self) {
        self.version += 1;
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
    type State = State;

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
            Self::PostAdded(ref e) => e.redmaple_id(),
            Self::PostPublished(ref e) => e.redmaple_id(),
            Self::PostModed(ref e) => e.redmaple_id(),
            Self::PostDeleted(ref e) => e.redmaple_id(),
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

    fn has_the_same_contents(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Created(a), Self::Created(b)) => a.redmaple_id() == b.redmaple_id(),
            (Self::PostAdded(a), Self::PostAdded(b)) => a.post() == b.post(),
            (Self::PostDeleted(a), Self::PostDeleted(b)) => a.post_id() == b.post_id(),
            (Self::PostModed(a), Self::PostModed(b)) => {
                a.post_id() == b.post_id() && a.new_mod() == b.new_mod()
            }
            (Self::PostPublished(a), Self::PostPublished(b)) => a.post_id() == b.post_id(),
            (_, _) => false,
        }
    }

    fn apply_to(&self, state: &mut Self::State) -> Result<(), Self::EventGroupError> {
        match self {
            Self::Created(c) => {
                state.set_view(c.view_mode().clone());
                state.increment_version();
                Ok(())
            }
            Self::PostAdded(c) => {
                state.add_dialog(Dialog::from(c.post()));
                state.increment_version();
                Ok(())
            }
            Self::PostPublished(c) => {
                state.publish_dialog(c.id(), *c.created())?;
                state.increment_version();
                Ok(())
            }
            Self::PostModed(c) => {
                state.mode_dialog(c.id(), c.new_mod().clone())?;
                state.increment_version();
                Ok(())
            }
            Self::PostDeleted(c) => {
                state.unpublish_dialog(c.id())?;
                state.increment_version();
                Ok(())
            }
        }
    }
}

impl Argument {
    /// Creates a new instance of `Argument::Created`
    #[must_use]
    pub fn new_create_event() -> Self {
        Self::Created(Created::new(views::Views::Blog(BlogMode::Text), ID::new()))
    }
}
