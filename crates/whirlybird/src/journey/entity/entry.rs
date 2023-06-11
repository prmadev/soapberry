//! [`entry`] module contains logic about a uesr entry

use std::time::SystemTime;

use redmaple::id::{IDGiver, ID};

use super::{body::Body, title::Title};

/// [`Entry`] contains information related to an specific user entry
#[derive(Clone, Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct Entry {
    /// The unique [`ID`] of certain entry.
    id: ValidEntryID,

    /// The time it was created.
    time_created: SystemTime,

    /// [`Title`] of the [`Entry`]
    title: Option<Title>,

    /// [`Body`] of the [`Entry`]
    body: Option<Body>,
}

impl Entry {
    /// `new` creates a new instance of [`Entry`]
    #[must_use]
    pub const fn new(
        id: ID,
        time_created: SystemTime,
        title: Option<Title>,
        body: Option<Body>,
    ) -> Self {
        Self {
            id: ValidEntryID(id),
            time_created,
            title,
            body,
        }
    }

    /// returns the [`Title`] if it is there
    #[must_use]
    pub const fn title(&self) -> &Option<Title> {
        &self.title
    }

    /// returns the [`Body`] if it is there
    #[must_use]
    pub const fn body(&self) -> &Option<Body> {
        &self.body
    }
}

/// A thin wrapper around [`ID`] that validates that the [`ID`] is coming from an [`Entry`]
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
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
