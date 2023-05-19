impl TryFrom<crate::grpc_definitions::GetEntriesRequest>
    for crate::domain::messages::queries::get_entries::GetEntries
{
    type Error = ToDomainGetEntriesError;

    fn try_from(_value: crate::grpc_definitions::GetEntriesRequest) -> Result<Self, Self::Error> {
        todo!()
    }
}

/// Errors that may happen while converting [`CreateEntryRequest`] to [`CreateEntry`]
#[derive(Debug, Clone, thiserror::Error)]
pub enum ToDomainGetEntriesError {}
