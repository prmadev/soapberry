impl TryFrom<crate::grpc_definitions::CreateEntryRequest>
    for crate::domain::messages::commands::create_entry::CreateEntry
{
    type Error = ToDomainCreateEntryError;

    fn try_from(value: crate::grpc_definitions::CreateEntryRequest) -> Result<Self, Self::Error> {
        let entry_title = match value.entry_title {
            Some(t) => Some(whirlybird::journey::entity::title::Title::try_from(t)?),
            None => None,
        };

        let body = match value.body {
            Some(b) => Some(whirlybird::journey::entity::body::Body::try_from(b)?),
            None => None,
        };

        Ok(crate::domain::messages::commands::create_entry::CreateEntry::new(entry_title, body))
    }
}
/// Errors that may happen while converting [`CreateEntryRequest`] to [`CreateEntry`]
#[derive(Debug, Clone, thiserror::Error)]
pub enum ToDomainCreateEntryError {
    /// title could not be built
    #[error("title could not be build: {0}")]
    TitleBuildingError(#[from] crate::grpc_definitions::title::TitleConversionError),

    /// body could not be built
    #[error("body could not be build: {0}")]
    BodyBuildingError(#[from] crate::grpc_definitions::body::BodyConversionError),

    /// body could not be built
    #[error("ID(s) could not be converted: {0:?}")]
    IDConversionErrors(Vec<crate::grpc_definitions::id::IDConversionError>),
}
