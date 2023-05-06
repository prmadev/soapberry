//! makes responds to the calls made

pub mod commands;

use redmaple::id::ID;

use tonic::{async_trait, Request, Response, Status};
use uuid::Uuid;

use crate::{
    domain::{self, messages::events::EventRepo},
    grpc_definitions::{
        journey_service_server::JourneyService, CreateEntryRequest, CreateEntryResponse,
    },
    persistence::structsy_store::events::entry_was_created::StructsyStore,
};

/// a service struct that is responds to the api calls
pub struct Service {
    db: structsy::Structsy,
    uuid_generator: fn() -> Uuid,
}

impl Service {
    /// creates a new journey
    pub const fn new(db: structsy::Structsy, uuid_generator: fn() -> Uuid) -> Self {
        Self { db, uuid_generator }
    }
}

#[async_trait]
impl JourneyService for Service {
    async fn create_entry(
        &self,
        request: Request<CreateEntryRequest>,
    ) -> Result<Response<CreateEntryResponse>, Status> {
        // command encoding
        let command =
            domain::messages::commands::create_entry::CreateEntry::try_from(request.into_inner())
                .map_err(|err| match err {
                crate::grpc_definitions::ToDomainCreateEntryError::TitleBuildingError(e) => {
                    Status::invalid_argument(e.to_string())
                }
                crate::grpc_definitions::ToDomainCreateEntryError::BodyBuildingError(e) => {
                    Status::invalid_argument(e.to_string())
                }
                crate::grpc_definitions::ToDomainCreateEntryError::IDConversionErrors(e) => {
                    Status::invalid_argument(format!("{e:?}"))
                }
            })?;
        let now = std::time::SystemTime::now();

        // Event Creation
        let event = domain::messages::events::entry_was_created::EntryWasCreated::new(
            ID::new((self.uuid_generator)()),
            now,
            ID::new((self.uuid_generator)()),
            command.entry_title().clone(),
            command.body().clone(),
        );

        // Storing event
        let store = StructsyStore::new(self.db.clone());

        if let Err(err) = store.append(event) {
            return Err(Status::internal(err.to_string()));
        }

        // Response
        Ok(Response::new(CreateEntryResponse {}))
    }
}
