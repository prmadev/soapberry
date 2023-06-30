//! [`entry`] module contains logic about a uesr entry

use std::fmt;

use redmaple::id::{Unique, ValidID, ID};

use super::body::Body;

/// [`Maple`] contains information related to an specific user entry
#[derive(Clone, Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct Maple {
    /// The unique [`ID`] of certain entry.
    id: ValidMapleID,

    /// [`Body`] of the [`Entry`]
    body: Body,
}

impl Maple {
    /// `new` creates a new instance of [`Entry`]
    #[must_use]
    pub const fn new(id: ID, body: Body) -> Self {
        Self {
            id: ValidMapleID(id),
            body,
        }
    }

    /// returns the [`Body`] if it is there
    #[must_use]
    pub const fn body(&self) -> &Body {
        &self.body
    }
}

/// A thin wrapper around [`ID`] that validates that the [`ID`] is coming from an [`Entry`]
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ValidMapleID(ID);

impl fmt::Display for ValidMapleID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner())
    }
}

impl fmt::LowerHex for ValidMapleID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:x}", self.inner())
    }
}

impl ValidID for ValidMapleID {
    /// exposes the inner [`ID`] of the [`Entry`]
    #[must_use]
    fn inner(&self) -> &ID {
        &self.0
    }

    fn into_id(self) -> ID {
        self.0
    }
}

impl Unique for Maple {
    type Valid = ValidMapleID;

    fn id(&self) -> &Self::Valid {
        &self.id
    }

    fn into_id(self) -> Self::Valid {
        self.id
    }
}

impl fmt::Display for Maple {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.body())
    }
}
