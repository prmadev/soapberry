//! [`link`] contains logic for [`Link`]

use std::fmt::Display;

use redmaple::RedMaple;

use crate::journey::{Event, EventWrapper, ValidMapleID};

/// [`Link`] is the holder of information between two valid objects
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Link {
    /// To which maple are we gazing at?
    to: ValidMapleID,
    /// tell me, what is this thing that we are looking at?
    explanation: String,
}

impl Display for Link {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.to, self.explanation)
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
                    });
                    accu
                }
            },
        );
        Self(links)
    }
}
