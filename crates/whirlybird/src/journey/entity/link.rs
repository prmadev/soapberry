//! [`link`] contains logic for [`Link`]

use std::fmt::Display;

use redmaple::{
    id::{Unique, ValidID, ID},
    RedMaple,
};

use crate::journey::{Event, EventWrapper, ValidMapleID};

/// [`Link`] is the holder of information between two valid objects
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Link {
    /// identifier
    id: ValidLinkID,
    /// To which maple are we gazing at?
    to: ValidMapleID,
    /// tell me, what is this thing that we are looking at?
    explanation: String,
}

impl Display for Link {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "LINK ID:\t{}\tNOTE ID:\t{}\tREASON:\t{}",
            self.id.inner(),
            self.to,
            self.explanation
        )
    }
}
impl Unique for Link {
    type Valid = ValidLinkID;

    fn id(&self) -> &Self::Valid {
        &self.id
    }

    fn into_id(self) -> Self::Valid {
        self.id
    }
}

/// a valid for of ID of a link
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ValidLinkID(pub(super) ID);

impl ValidID for ValidLinkID {
    #[must_use]
    fn inner(&self) -> &ID {
        &self.0
    }

    #[must_use]
    fn into_inner(self) -> ID {
        self.0
    }
}

/// Links holds a bunch of Links
/// it is just here to facilitate getting implementation of From<RedMaple<EventWrapper>>
pub struct Links(pub Vec<Link>);

impl From<&RedMaple<EventWrapper>> for Links {
    fn from(value: &RedMaple<EventWrapper>) -> Self {
        let links: Vec<Link> = value.events().iter().map(EventWrapper::data).fold(
            vec![],
            |mut accu, event| match event {
                Event::MapleCreated(_) | Event::MapleBodyUpdated(_, _) => accu,
                Event::LinkAdded(l) => {
                    accu.push(Link {
                        to: l.0.clone(),
                        explanation: l.1.clone(),
                        id: ValidLinkID(l.2.clone()),
                    });
                    accu
                }
                Event::Dislinked(link_id) => {
                    accu.into_iter().filter(|l| !(l.id() == link_id)).collect()
                }
            },
        );
        Self(links)
    }
}
