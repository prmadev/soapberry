//! [`title`] contains the logic for a valid title for an [`Entry`]

/// [`Title`] is similar to [`Body`] in that it is a wrapper around simple [`String`] to
/// ensure that the text alway follows the domain rules.
#[derive(Clone, Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct Title(String);

impl Title {
    /// `build` checks if the the domain rules are being followed
    ///
    /// # Errors
    ///
    /// * [`TitleBuildingError::TextCannotBeEmpty`] can be returned in-case of empty [`String`].
    pub fn build(text: String) -> Result<Self, BuildingError> {
        if text.is_empty() {
            return Err(BuildingError::TextCannotBeEmpty);
        };

        Ok(Self(text))
    }

    /// The inner string  of [`Title`]
    #[must_use]
    pub const fn inner(&self) -> &String {
        &self.0
    }
}
/// errors that may arise while making a [`Title`]
#[derive(Debug, Clone, thiserror::Error)]
pub enum BuildingError {
    /// the title text should have other characters.
    /// in case you are looking for not adding a [`Title`] wrap it in `Option<Title>` and return
    /// [`Option::None`]
    #[error("the title text should have other characters")]
    TextCannotBeEmpty,
}
