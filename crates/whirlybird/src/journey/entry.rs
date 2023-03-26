//! [`entry`] module contains logic about a uesr entry

use std::time::SystemTime;

use getset_scoped::Getters;
use redmaple::id::ID;

use super::{Body, Link, Title, ValidJourneyID};

/// [`Entry`] contains information related to an specific user entry
#[derive(Clone, Debug, Getters, PartialEq, Eq)]
pub struct Entry {
    /// The unique [`ID`] of certain entry.
    #[getset(get = "pub")]
    id: ValidEntryID,

    /// The time it was created.
    #[getset(get = "pub")]
    time_created: SystemTime,

    /// [`Title`] of the [`Entry`]
    #[getset(get = "pub")]
    title: Option<Title>,

    /// [`Body`] of the [`Entry`]
    #[getset(get = "pub")]
    body: Option<Body>,

    /// list of [`Link`] s from this [`Entry`]
    #[getset(get = "pub")]
    links: Vec<Link>,

    /// [`Journey`] s that this [`Entry`] is on
    #[getset(get = "pub")]
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
