use std::{
    fmt::{Debug, Display},
    time::SystemTime,
};

use redmaple::id::ID;

/// Content type sets the mode of each content.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Mode {
    /// The main post of the redmaple
    HeadPost,
    /// Answer the redmaple, or optionally, answer to another content of the same redmaple
    Conversation(Option<ID>),
    /// A new edition for the headpost
    Edition,
}

/// Content holds the different forms of content
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Post<T: Sized + Display, P: Sized + Display> {
    /// A Text content is a string.
    Text {
        id: ID,
        date: SystemTime,
        content: T,
    },
    /// A Text content is a subscription to a picture stream.
    Picture {
        id: ID,
        date: SystemTime,
        content: P,
    },
}

impl Post<String, String> {
    pub const fn id(&self) -> &ID {
        match self {
            Self::Text { id, .. } | Self::Picture { id, .. } => id,
        }
    }
    pub const fn date(&self) -> &SystemTime {
        match self {
            Self::Text { date, .. } | Self::Picture { date, .. } => date,
        }
    }

    pub const fn content(&self) -> &String {
        match self {
            Self::Text { content, .. } | Self::Picture { content, .. } => content,
        }
    }
}
