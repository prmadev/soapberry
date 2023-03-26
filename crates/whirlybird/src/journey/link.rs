//! [`link`] contains logic for [`Link`]

use std::time::SystemTime;

use getset_scoped::Getters;
use redmaple::id::ID;

use super::{body::Body, entry::ValidEntryID, title::Title};

/// A thin wrapper around [`ID`] that validates that the [`ID`] is coming from an [`Link`]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidLinkID(ID);

impl ValidLinkID {
    /// exposes the inner [`ID`] of the [`Link`]
    #[must_use]
    pub const fn inner(&self) -> &ID {
        &self.0
    }
}

/// [`Link`] is the holder of information between two valid objects
#[derive(Clone, Debug, Getters, PartialEq, Eq)]
pub struct Link {
    /// The unique [`ID`] of certain [`Link`].
    #[getset(get = "pub")]
    id: ValidLinkID,

    /// The time it was created.
    #[getset(get = "pub")]
    time_created: SystemTime,

    /// The unique [`ID`] of certain the [`Entry`] which the [`Link`] is started from.
    #[getset(get = "pub")]
    from_id: ValidEntryID,

    /// The unique [`ID`] of certain the [`Entry`] which the [`Link`] is pointing to.
    #[getset(get = "pub")]
    to_id: ValidEntryID,

    /// [`Title`] of the [`Entry`]
    #[getset(get = "pub")]
    title: Title,

    /// [`Body`] of the [`Entry`]
    #[getset(get = "pub")]
    body: Body,
}

impl Link {
    /// creates a new instance of [`Link`]
    #[must_use]
    pub const fn new(
        id: ID,
        time_created: SystemTime,
        from_id: ValidEntryID,
        to_id: ValidEntryID,
        title: Title,
        body: Body,
    ) -> Self {
        Self {
            id: ValidLinkID(id),
            time_created,
            from_id,
            to_id,
            title,
            body,
        }
    }
}
