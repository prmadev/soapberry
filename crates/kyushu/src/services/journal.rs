//! makes responds to the calls made

pub mod commands;

use redmaple::id::ID;
use std::time::SystemTime;
use structsy::StructsyTx;
use tonic::{async_trait, Request, Response, Status};
use uuid::Uuid;
use whirlybird::journey::Journey;

use crate::{
    grpc_definitions::{
        journey_service_server::JourneyService, CreateEntryRequest, CreateEntryResponse,
    },
    persistence::structsy_store::{
        persisted::entry_was_created::EntryWasCreated, queries::query_journey_was_created,
    },
};

use self::commands::create_entry::{entry_creator, Error};

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
        let req = request.into_inner();

        let creator = entry_creator(
            |id| {
                if let Some(k) = query_journey_was_created(self.db.clone())(id)
                    .fetch()
                    .next()
                {
                    return Journey::try_from(k.1).map_err(|_e| Error::CouldNotConvert);
                };

                Err(Error::CouldNotFindItenInDatabase)
            },
            |n| n.id.ok_or(Error::JouneyIDNotFound).map(|j| j.id),
        );

        let entry = creator(
            ID::new((self.uuid_generator)()),
            ID::new((self.uuid_generator)()),
            ID::new(uuid::Uuid::nil()),
            SystemTime::now(),
            req,
        )
        .map_err(|e| match e {
            Error::TitleOrBodyCannotBeBuilt(err) => Status::invalid_argument(err.to_string()),
            Error::CouldNotBuildUUID(err) => Status::invalid_argument(err.to_string()),
            Error::JouneyIDNotFound => Status::not_found("Journey ID was not provided"),
            Error::JouneyWasNotFound => {
                Status::not_found("journey was not not found inside the database")
            }
            Error::JourneyBatchProblems(err) => Status::unknown(format!("{err:?}")),
            Error::CouldNotConvert => Status::invalid_argument("Could not convert one the values"),
            Error::CouldNotFindItenInDatabase => {
                Status::not_found("could not find the item in the database")
            }
        })?;

        let persisting_event =
            EntryWasCreated::try_from(entry).map_err(|e| Status::internal(e.to_string()))?;

        let mut tx = self
            .db
            .begin()
            .map_err(|e| Status::internal(e.to_string()))?;

        tx.insert(&persisting_event).map_err(|e| {
            eprintln!("{e}");
            Status::internal(e.to_string())
        })?;

        tx.commit().map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(CreateEntryResponse {}))

        // TODO: make a MakeJournalEvent
        // TODO: retrieve journals from redmaple
    }
}
