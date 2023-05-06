//! `api` is a module wrapper to import `tonic-build` generated GRPC SDKs to be used by both client
//! and server binaries.

// the reason for this is that this module will import generated code which does not need to adhere
// to the standards of user-written code
#![allow(
    clippy::perf,
    clippy::style,
    clippy::pedantic,
    clippy::cargo,
    clippy::nursery,
    clippy::complexity,
    clippy::restriction,
    missing_docs
)]

use whirlybird::journey::{body, title};

use crate::domain;

tonic::include_proto!("health.v1");
tonic::include_proto!("journey.v1");

/// a set of helper function to deal with apis
pub mod helper {
    /// Converts a [`crate::api::Journey`]'s id  from `Option<String>` into `Result<String, E>`
    ///
    /// # Errors
    ///
    /// This function will return an error if if the id field is `Option::None`.
    #[allow(clippy::needless_pass_by_value)]
    pub fn journey_id_extractor<E>(
        x: crate::grpc_definitions::Journey,
        when_id_not_found: E,
    ) -> Result<String, E> {
        Ok(x.id.ok_or(when_id_not_found)?.id)
    }
}

impl TryFrom<Title> for title::Title {
    type Error = TitleConversionError;

    fn try_from(value: Title) -> Result<Self, Self::Error> {
        Ok(Self::build(value.content).map_err(TitleConversionError::TitleBuildingError)?)
    }
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum TitleConversionError {
    /// Title could not be build
    #[error("Title could not be build for the text: {0}")]
    TitleBuildingError(#[from] title::BuildingError),
}

impl TryFrom<Body> for whirlybird::journey::body::Body {
    type Error = BodyConversionError;

    fn try_from(value: Body) -> Result<Self, Self::Error> {
        Ok(Self::build(value.content).map_err(BodyConversionError::BodyBuildingError)?)
    }
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum BodyConversionError {
    /// Title could not be build
    #[error("Body could not be build for the text: {0}")]
    BodyBuildingError(#[from] body::BuildingError),
}

impl TryFrom<Id> for redmaple::id::ID {
    type Error = IDConversionError;

    fn try_from(value: Id) -> Result<Self, Self::Error> {
        Ok(Self::new(uuid::Uuid::parse_str(&value.id)?))
    }
}

impl TryFrom<CreateEntryRequest> for domain::messages::commands::create_entry::CreateEntry {
    type Error = ToDomainCreateEntryError;

    fn try_from(value: CreateEntryRequest) -> Result<Self, Self::Error> {
        let entry_title = match value.entry_title {
            Some(t) => Some(whirlybird::journey::title::Title::try_from(t)?),
            None => None,
        };

        let body = match value.body {
            Some(b) => Some(whirlybird::journey::body::Body::try_from(b)?),
            None => None,
        };

        let (_, errors): (Vec<_>, Vec<_>) = value
            .journeys_to_be_added
            .iter()
            .filter_map(|j| j.id.clone())
            .map(|i| redmaple::id::ID::try_from(i))
            .partition(|i| i.is_ok());

        let errors: Vec<IDConversionError> = errors
            .into_iter()
            .filter_map(std::result::Result::err)
            .collect();

        if !errors.is_empty() {
            return Err(ToDomainCreateEntryError::IDConversionErrors(errors));
        };

        Ok(domain::messages::commands::create_entry::CreateEntry::new(
            entry_title,
            body,
        ))
    }
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum IDConversionError {
    /// Title could not be build
    #[error("ID could not be build for the string: {0}")]
    UUIDParsingError(#[from] uuid::Error),
}

/// Errors that may happen while converting [`CreateEntryRequest`] to [`CreateEntry`]
#[derive(Debug, Clone, thiserror::Error)]
pub enum ToDomainCreateEntryError {
    /// title could not be built
    #[error("title could not be build: {0}")]
    TitleBuildingError(#[from] TitleConversionError),

    /// body could not be built
    #[error("body could not be build: {0}")]
    BodyBuildingError(#[from] BodyConversionError),

    /// body could not be built
    #[error("ID(s) could not be converted: {0:?}")]
    IDConversionErrors(Vec<IDConversionError>),
}
