//! [`title`] contains the logic for a valid title for an [`Entry`]

use super::DomainError;

/// [`Title`] is similar to [`Body`] in that it is a wrapper around simple [`String`] to
/// ensure that the text alway follows the domain rules.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Title(String);

impl Title {
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

    /// The inner string  of [`Title`]
    #[must_use]
    pub const fn inner(&self) -> &String {
        &self.0
    }
}
