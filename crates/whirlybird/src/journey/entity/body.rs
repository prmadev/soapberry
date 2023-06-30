//! [`body`] module contains information about the text body of an entry

use std::fmt;

use redmaple::RedMaple;

use crate::journey::{Event, EventWrapper};

/// `Body` is a wrapper around simple [`String`] to ensure that the text alway follows the domain rules
#[derive(Clone, Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub enum Body {
    /// A simple line text that should probably take only one line  
    OneLineText(String),
}

impl Default for Body {
    fn default() -> Self {
        Self::OneLineText(String::default())
    }
}

impl fmt::Display for Body {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OneLineText(t) => write!(f, "{t}"),
        }
    }
}

/// errors that may arise while making a [`Title`]
#[derive(Debug, Clone, thiserror::Error)]
pub enum BuildingError {
    /// the body text should have other characters.
    /// in case you are looking for not adding a [`Body`] wrap it in `Option<Body>` and return
    /// [`Option::None`]
    #[error("the body text should have other characters")]
    TextCannotBeEmpty,
}

impl TryFrom<String> for Body {
    type Error = BuildingError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(BuildingError::TextCannotBeEmpty);
        };

        Ok(Self::OneLineText(value))
    }
}

impl From<RedMaple<EventWrapper>> for Body {
    fn from(value: RedMaple<EventWrapper>) -> Self {
        value
            .events()
            .iter()
            .map(EventWrapper::data)
            .fold(Self::default(), |_accu, event| match event {
                Event::MapleCreated(m) => m.body().clone(),
                Event::MapleBodyUpdated(_, b) => b.clone(),
            })
    }
}
