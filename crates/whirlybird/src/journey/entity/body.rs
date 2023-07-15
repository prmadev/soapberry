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
                Event::Dislinked(_) | Event::LinkAdded(_) => accu,
            })
    }
}

#[cfg(test)]
mod tests {
    use time::OffsetDateTime;

    use crate::journey::{Maple, ValidMapleID};

    use super::*;

    const VALID_BODY_TEXT: &str = "Some Valid text";
    const VALID_BODY_TEXT_TWO: &str = "Some Other Valid text";
    const EMPTY_BODY_TEXT: &str = "";

    #[test]
    fn test_empty_body_from_string() -> Result<(), String> {
        let empty_string = Body::try_from(EMPTY_BODY_TEXT.to_string());
        match empty_string {
            Ok(_) => Err("body should never be empty".to_owned()),
            Err(err) => match err {
                BuildingError::TextCannotBeEmpty => Ok(()),
            },
        }
    }

    #[test]
    fn test_valid_body_from_string() -> Result<(), String> {
        let valid_string = Body::try_from(VALID_BODY_TEXT.to_string());
        match valid_string {
            Ok(_) => Ok(()),
            Err(err) => Err(format!(
                "a valid body should not get error. but instead it got one: {err}"
            )),
        }
    }

    #[test]
    fn test_body_from_redmaple() -> Result<(), String> {
        let first_event = {
            let this_event_time = OffsetDateTime::now_utc();
            let new_maple = Maple::new(
                this_event_time.into(),
                Body::try_from(VALID_BODY_TEXT.to_string()).map_err(|e| {
                    format!("a valid body should not get error. but instead it got one: {e}")
                })?,
            );

            EventWrapper::new(
                this_event_time.into(),
                this_event_time,
                Event::MapleCreated(new_maple),
            )
        };

        let the_redmaple = RedMaple::new(vec![first_event]);
        let _ = match Body::try_from(&the_redmaple) {
            Ok(b) => match b {
                Body::OneLineText(text) => {
                    if text == VALID_BODY_TEXT {
                        Ok(())
                    } else {
                        Err(format!("wanted '{VALID_BODY_TEXT}', instead go '{text}'"))
                    }
                }
            },
            Err(e) => Err(format!(
                "Should be able to get a body. but instead it got one: {e}"
            )),
        }?;

        let second_event = {
            let this_event_time = OffsetDateTime::now_utc();
            let valid_maple_id = ValidMapleID::try_from(&the_redmaple).map_err(|err| format!("I redmaple with events should be able to give out valid maple ids but it could not. instead got: {err}"))?;

            EventWrapper::new(
                this_event_time.into(),
                this_event_time,
                Event::MapleBodyUpdated(
                    valid_maple_id,
                    Body::try_from(VALID_BODY_TEXT_TWO.to_owned()).map_err(|err| {
                        format!("Should be able to get a body. but instead it got one: {err}")
                            .to_owned()
                    })?,
                ),
            )
        };

        let redmaple_v2 = the_redmaple.into_appended(second_event);
        _ = match Body::try_from(&redmaple_v2) {
            Ok(b) => match b {
                Body::OneLineText(text) => {
                    if text == VALID_BODY_TEXT_TWO.to_string() {
                        Ok(())
                    } else {
                        Err(format!(
                            "wanted '{VALID_BODY_TEXT_TWO}', instead got '{text}'"
                        ))
                    }
                }
            },
            Err(e) => Err(format!(
                "Should be able to get a body. but instead it got: {e}"
            )),
        }?;

        Ok(())
    }
}
