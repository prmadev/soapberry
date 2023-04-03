//! [`entry`] module contains logic about a uesr entry

use std::time::SystemTime;

use redmaple::id::{IDGiver, ID};

use super::{Body, Link, Title, ValidJourneyID};

/// [`Entry`] contains information related to an specific user entry
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Entry {
    /// The unique [`ID`] of certain entry.
    id: ValidEntryID,

    /// The time it was created.
    time_created: SystemTime,

    /// [`Title`] of the [`Entry`]
    title: Option<Title>,

    /// [`Body`] of the [`Entry`]
    body: Option<Body>,

    /// list of [`Link`] s from this [`Entry`]
    links: Vec<Link>,

    /// [`Journey`] s that this [`Entry`] is on
    journeys: Vec<ValidJourneyID>,
}

impl Entry {
    /// `new` creates a new instance of [`Entry`]
    #[must_use]
    pub const fn new(
        id: ID,
        time_created: SystemTime,
        title: Option<Title>,
        body: Option<Body>,
        links: Vec<Link>,
        journeys: Vec<ValidJourneyID>,
    ) -> Self {
        Self {
            id: ValidEntryID(id),
            time_created,
            title,
            body,
            links,
            journeys,
        }
    }
}

/// A thin wrapper around [`ID`] that validates that the [`ID`] is coming from an [`Entry`]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidEntryID(ID);

impl ValidEntryID {
    /// exposes the inner [`ID`] of the [`Entry`]
    #[must_use]
    pub const fn inner(&self) -> &ID {
        &self.0
    }
}

impl IDGiver for Entry {
    type Valid = ValidEntryID;

    fn id(&self) -> &Self::Valid {
        &self.id
    }

    fn into_id(self) -> Self::Valid {
        self.id
    }
}
