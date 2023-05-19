//!  implementation for converting bodies

impl TryFrom<crate::grpc_definitions::Body> for whirlybird::journey::entity::body::Body {
    type Error = BodyConversionError;

    fn try_from(value: crate::grpc_definitions::Body) -> Result<Self, Self::Error> {
        Ok(Self::build(value.content).map_err(BodyConversionError::BodyBuildingError)?)
    }
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum BodyConversionError {
    /// Title could not be build
    #[error("Body could not be build for the text: {0}")]
    BodyBuildingError(#[from] whirlybird::journey::entity::body::BuildingError),
}
