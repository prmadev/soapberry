//! Welcome to the fascinating realm of the [`body`] module, where profound insights into the text body of an entry await.

use std::fmt;

use redmaple::RedMaple;

use crate::journey::{Event, EventWrapper};

/// Behold the profound essence of the `Body`, the sacred vessel
/// that encapsulates the diverse manifestations of description and the very essence of an entry's main point.
#[derive(Clone, Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub enum Body {
    /// Prepare yourself for the simplicity embodied in the OneLineText variant, where a mere single line of text resides.
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

/// Let us venture into the realm of potential errors that may emerge during the creation of a [body], where mysteries of existence unravel.
#[derive(Debug, Clone, thiserror::Error)]
pub enum BuildingError {
    /// Reflect upon the wisdom bestowed upon us, for it is decreed that the body text should not be devoid of characters.
    /// In the event that you seek to abstain from adorning the body with a [Body] wrap,
    /// contemplate enshrining it within the sanctity of Option<Body> and bestowing upon the world [Option::None].
    #[error("The body text must contain characters to effectively convey the intricacies of the topic at hand.")]
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

impl From<&RedMaple<EventWrapper>> for Body {
    fn from(value: &RedMaple<EventWrapper>) -> Self {
        value
            .events()
            .iter()
            .map(EventWrapper::data)
            .fold(Self::default(), |accu, event| match event {
                Event::MapleCreated(m) => m.body().clone(),
                Event::MapleBodyUpdated(_, b) => b.clone(),
                Event::LinkAdded(_) => accu,
            })
    }
}
