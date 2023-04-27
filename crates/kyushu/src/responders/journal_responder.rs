#![allow(unused)]
//! makes responds to the calls made
use std::time::SystemTime;

use redmaple::id::ID;
use structsy::StructsyTx;
use tonic::{async_trait, Request, Response, Status};
use uuid::Uuid;
use whirlybird::journey::Journey;

use crate::{
    api::{journey_service_server::JourneyService, CreateEntryRequest, CreateEntryResponse},
    domain::{entry_creator, JournalEventError},
    persistence::structsy_store::{
        persisted::{self, EntryWasCreated},
        queries::query_journey_was_created,
    },
};

/// a service struct that is responds to the api calls
pub struct JournalResponder {
    db: structsy::Structsy,
    uuid_generator: fn() -> Uuid,
}

#[async_trait]
impl JourneyService for JournalResponder {
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
                    return Journey::try_from(k.1).map_err(|_e| JournalEventError::CouldNotConvert);
                };

                Err(JournalEventError::CouldNotFindItenInDatabase)
            },
            |n| {
                n.id.ok_or(JournalEventError::JouneyIDNotFound)
                    .map(|j| j.id)
            },
        );

        let entry = creator(
            ID::new((self.uuid_generator)()),
            ID::new((self.uuid_generator)()),
            ID::new(uuid::Uuid::nil()),
            SystemTime::now(),
            req,
        )
        .map_err(|e| match e {
            JournalEventError::TitleOrBodyCannotBeBuilt(err) => {
                Status::invalid_argument(err.to_string())
            }
            JournalEventError::CouldNotBuildUUID(err) => Status::invalid_argument(err.to_string()),
            JournalEventError::JouneyIDNotFound => Status::not_found("Journey ID was not provided"),
            JournalEventError::JouneyWasNotFound => {
                Status::not_found("journey was not not found inside the database")
            }
            JournalEventError::JourneyBatchProblems(err) => Status::unknown(format!("{err:?}")),
            JournalEventError::CouldNotConvert => {
                Status::invalid_argument("Could not convert one the values")
            }
            JournalEventError::CouldNotFindItenInDatabase => {
                Status::not_found("could not find the item in the database")
            }
        })?;

        let persisting_event = persisted::EntryWasCreated::try_from(entry)
            .map_err(|e| Status::internal(e.to_string()))?;

        let mut tx = self
            .db
            .begin()
            .map_err(|e| Status::internal(e.to_string()))?;

        tx.insert::<persisted::EntryWasCreated>(&persisting_event)
            .map_err(|e| Status::internal(e.to_string()))?;
        tx.commit().map_err(|e| Status::internal(e.to_string()))?;
        Ok(Response::new(CreateEntryResponse {}))

        // TODO: make a MakeJournalEvent
        // TODO: retrieve journals from redmaple
    }
}
