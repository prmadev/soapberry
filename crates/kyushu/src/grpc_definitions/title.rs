//!  implementation for converting titles

impl TryFrom<crate::grpc_definitions::Title> for whirlybird::journey::entity::title::Title {
    type Error = TitleConversionError;

    fn try_from(value: crate::grpc_definitions::Title) -> Result<Self, Self::Error> {
        Ok(Self::build(value.content).map_err(TitleConversionError::TitleBuildingError)?)
    }
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum TitleConversionError {
    /// Title could not be build
    #[error("Title could not be build for the text: {0}")]
    TitleBuildingError(#[from] whirlybird::journey::entity::title::BuildingError),
}
