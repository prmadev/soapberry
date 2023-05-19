//! implementation for conversion of [`Id`]
impl TryFrom<crate::grpc_definitions::Id> for redmaple::id::ID {
    type Error = IDConversionError;

    fn try_from(value: crate::grpc_definitions::Id) -> Result<Self, Self::Error> {
        Ok(Self::new(uuid::Uuid::parse_str(&value.id)?))
    }
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum IDConversionError {
    /// Title could not be build
    #[error("ID could not be build for the string: {0}")]
    UUIDParsingError(#[from] uuid::Error),
}
