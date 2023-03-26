//! [`body`] module contains information about the text body of an entry

/// `Body` is a wrapper around simple [`String`] to ensure that the text alway follows the domain rules
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Body(String);

impl Body {
    /// `build` checks if the the domain rules are being followed
    ///
    /// # Errors
    ///
    /// * [`JourneyError::TextCannotBeEmpty`] can be returned in-case of empty [`String`].
    pub fn build(text: String) -> Result<Self, DomainError> {
        if text.is_empty() {
            return Err(DomainError::TextCannotBeEmpty);
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
/// Errors that are resulted from functions  and emthods inside [`journey`]
#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub enum DomainError {
    /// For when a text field should contain 1 or more characters
    #[error("Text Cannot have 0 length")]
    TextCannotBeEmpty,
}
