//! these are the persistence <-> redmaple operators

use std::time::SystemTime;

use crate::api::CreateEntryRequest;
use redmaple::id::{result_id, ID};
use thiserror::Error;
use whirlybird::journey::{
    body::Body, entry::Entry, title::Title, DomainError, Journal, JournalEvent, Journey,
    ValidJourneyID,
};

/// Returns a list [`ValidJourneyID`]s given a list of `String`s
/// you should note that this function is lossy with regards to errors it gets
///
/// # Errors
///
/// This function will return an error if the underlying `journey_finder` errors. It will however, convert it into [`JournalEventError::JourneyBatchProblems`].
pub fn valid_journey_id_list(
    journey_id_list: Vec<String>,
    journey_finder: impl FnMut(String) -> Result<Journey, JournalEventError>,
    journey_id_extractor: impl Fn(Result<Journey, JournalEventError>) -> Option<ValidJourneyID>,
) -> Result<Vec<ValidJourneyID>, JournalEventError> {
    let (journey_inquieries, errs): (Vec<_>, Vec<_>) = journey_id_list
        .into_iter()
        .map(journey_finder)
        .partition(Result::is_ok);

    if !errs.is_empty() {
        return Err(JournalEventError::JourneyBatchProblems(
            errs.into_iter().filter_map(Result::err).collect(),
        ));
    };

    let journeys: Vec<ValidJourneyID> = journey_inquieries
        .into_iter()
        .filter_map(journey_id_extractor)
        .collect();
    Ok(journeys)
}

///  Generates a new  function that creates a [`Journal::EntryCreated`] event
///
/// # Errors
///
/// This function will return an error if in the process of parsing the request or retrieving journals
/// an error accures.
pub fn entry_creator<Q, A>(
    query: Q,
    api_journey_id_extractor: A,
) -> impl FnOnce(ID, ID, ID, SystemTime, CreateEntryRequest) -> Result<JournalEvent, JournalEventError>
where
    Q: Fn(String) -> Result<Journey, JournalEventError>,
    A: Fn(crate::api::Journey) -> Result<String, JournalEventError>,
{
    move |entry_id: ID,
          new_event_id: ID,
          journal_id: ID,
          now: SystemTime,
          request: CreateEntryRequest|
          -> Result<JournalEvent, JournalEventError> {
        let (raw_ids, errs): (Vec<_>, Vec<_>) = request
            .journeys_to_be_added
            .into_iter()
            .map(api_journey_id_extractor)
            .partition(Result::is_ok);

        if !errs.is_empty() {
            return Err(JournalEventError::JourneyBatchProblems(
                errs.into_iter().filter_map(Result::err).collect(),
            ));
        };

        let journeys = valid_journey_id_list(
            raw_ids.into_iter().filter_map(Result::ok).collect(),
            query,
            result_id,
        )?;

        let title = match request.entry_title {
            Some(t) => Some(Title::build(t.content)?),
            None => None,
        };

        let body = match request.body {
            Some(b) => Some(Body::build(b.content)?),
            None => None,
        };

        let data = Journal::EntryCreated(Entry::new(entry_id, now, title, body, vec![], journeys));
        Ok(JournalEvent::new(new_event_id, now, journal_id, data))
    }
}

/// Public error type related to Journal service
#[derive(Error, Debug, Clone)]
pub enum JournalEventError {
    /// errors related to title or body creation from input
    #[error("Title/body cannot get built: {0}")]
    TitleOrBodyCannotBeBuilt(#[from] DomainError),

    /// errors related to parsing of UUID from string
    #[error("ID cannot get parsed: {0}")]
    CouldNotBuildUUID(#[from] uuid::Error),

    /// should not happen. but if happens it is because journey ID was not provided.
    #[error("Journey ID was not provided")]
    JouneyIDNotFound,

    /// happens when an invalid uuid to a journey was given
    #[error("The provided journey ID does not match any provided journeys")]
    JouneyWasNotFound,

    /// used when checking a batch of journey ID to see if they are valid
    #[error("Found errors related to finding journeys: {0:?}")]
    JourneyBatchProblems(Vec<JournalEventError>),

    /// problems converting JourneyCreation into Journey
    #[error("Could not convert")]
    CouldNotConvert,
}
