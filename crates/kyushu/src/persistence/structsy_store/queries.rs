//! helpers for abstracting over side-effectful queries
//!

use structsy::{Structsy, StructsyQuery};

use super::persisted::{JourneyWasCreated, JourneyWasCreatedQuery};

/// creates a side effectfull query for [`JourneyWasCreated`]
pub fn query_journey_was_created(
    db: Structsy,
) -> impl Fn(String) -> StructsyQuery<JourneyWasCreated> {
    move |id: String| -> StructsyQuery<JourneyWasCreated> {
        db.query::<JourneyWasCreated>().search_by_journey_id(&id)
    }
}
