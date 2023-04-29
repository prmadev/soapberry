//!indicated that a journey was created at some point in time;

use std::time::SystemTime;

use redmaple::id::{IDGiver, ID};
use whirlybird::journey::{body::Body, link::Link, title::Title, ValidJourneyID};

use super::ValidEventID;

/// indicates that an [`Entry`] was created
pub struct EntryWasCreated {
    event_id: ValidEventID,
    time_created: SystemTime,
    entry_id: ID,
    title: Option<Title>,
    body: Option<Body>,
    links: Vec<Link>,
    journeys: Vec<ValidJourneyID>,
}

impl EntryWasCreated {
    /// creates a new instance of [`EntryWasCreated`]
    #[must_use]
    pub fn new(
        event_id: ID,
        time_created: SystemTime,
        entry_id: ID,
        title: Option<Title>,
        body: Option<Body>,
        links: Vec<Link>,
        journeys: Vec<ValidJourneyID>,
    ) -> Self {
        Self {
            event_id: ValidEventID(event_id),
            time_created,
            entry_id,
            title,
            body,
            links,
            journeys,
        }
    }

    /// returns the time that the event was created
    #[must_use]
    pub const fn time_created(&self) -> SystemTime {
        self.time_created
    }

    /// returns the [`ID`] that is created to ["entry"]
    #[must_use]
    pub const fn entry_id(&self) -> &ID {
        &self.entry_id
    }

    /// returns the title of the entry
    #[must_use]
    pub const fn title(&self) -> Option<&Title> {
        self.title.as_ref()
    }

    /// returns the title of the entry
    #[must_use]
    pub const fn body(&self) -> Option<&Body> {
        self.body.as_ref()
    }

    /// returns the links of this entry
    #[must_use]
    pub fn links(&self) -> &[Link] {
        self.links.as_ref()
    }

    /// reutrns the journeys that this is on
    #[must_use]
    pub fn journeys(&self) -> &[ValidJourneyID] {
        self.journeys.as_ref()
    }
}

impl IDGiver for EntryWasCreated {
    type Valid = ValidEventID;

    #[must_use]
    fn id(&self) -> &Self::Valid {
        &self.event_id
    }

    #[must_use]
    fn into_id(self) -> Self::Valid {
        self.event_id
    }
}
