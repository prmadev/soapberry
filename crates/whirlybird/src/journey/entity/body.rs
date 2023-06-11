//! [`body`] module contains information about the text body of an entry

/// `Body` is a wrapper around simple [`String`] to ensure that the text alway follows the domain rules
#[derive(Clone, Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct Body(String);

impl Body {
    /// `build` checks if the the domain rules are being followed
    ///
    /// # Errors
    ///
    /// * [`JourneyError::TextCannotBeEmpty`] can be returned in-case of empty [`String`].
    pub fn build(text: String) -> Result<Self, BuildingError> {
        if text.is_empty() {
            return Err(BuildingError::TextCannotBeEmpty);
        };

        Ok(Self(text))
    }

    /// the inner string of [`Body`]
    #[must_use]
    pub const fn inner(&self) -> &String {
        &self.0
    }

    /// Return the inner string of [`Body`] and consumes itself in the process
    #[must_use]
    #[allow(clippy::missing_const_for_fn)] // currently a destructor method cannot be const
    pub fn into_inner(self) -> String {
        self.0
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
