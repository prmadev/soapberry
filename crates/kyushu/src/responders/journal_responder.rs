#![allow(unused)]
//! makes responds to the calls made
use tonic::{async_trait, Request, Response};

use crate::{
    api::{journey_service_server::JourneyService, CreateEntryRequest, CreateEntryResponse},
    domain::entry_creator,
    persistence::structsy_store::queries::query_journey_was_created,
};

/// a service struct that is responds to the api calls
pub struct JournalResponder;

#[async_trait]
impl JourneyService for JournalResponder {
    async fn create_entry(
        &self,
        request: Request<CreateEntryRequest>,
    ) -> Result<Response<CreateEntryResponse>, tonic::Status> {
        let _a = request.into_inner();

        // let entry_creator(query_journey_was_created());

        // TODO: make a CreateEntryEvent
        // TODO: make a check for existence of that event
        // TODO: Persist that event
        // TODO: make a MakeJournalEvent
        // TODO: retrieve journals from redmaple

        todo!();
    }
}
