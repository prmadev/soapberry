//! it hold the logic of a contemplating journal.
//! it is difference from a zettelkasten-style journaling.
//! in that each node entry has a time associated with it.
//! and forms named relation ships.
//! these relationships form journeys
pub mod entity;

pub use entity::*;

pub mod event;
pub use event::*;

use crate::journey::event::ValidEventID;

use redmaple::{
    event_group::EventKind,
    id::{Unique, ValidID, ID},
    RedMaple,
};

/// [`JournelaEvent`] holds the meta data for [`Journal`] event
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct EventWrapper {
    event_id: ValidEventID,
    time: time::OffsetDateTime,
    data: Event,
}

impl TryFrom<&RedMaple<EventWrapper>> for ValidMapleID {
    type Error = IDGetterError;

    fn try_from(value: &RedMaple<EventWrapper>) -> Result<Self, Self::Error> {
        value
            .events()
            .iter()
            .fold(Option::None, |ac, m| match m.data() {
                Event::MapleCreated(mp) => Some(mp.id().clone()),
                Event::MapleBodyUpdated(_, _) => ac,
            })
            .ok_or(IDGetterError::NoEventsFound)
    }
}

/// error that happens as the result of failing to geto `ValidMapleID` from `RedMaple`
#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub enum IDGetterError {
    /// For when a text field should contain 1 or more characters
    #[error("RedMaple contains no events!")]
    NoEventsFound,
}

impl EventWrapper {
    /// this will create a new Journal event
    #[must_use]
    pub const fn new(event_id: ID, time: time::OffsetDateTime, data: Event) -> Self {
        Self {
            event_id: ValidEventID(event_id),
            time,
            data,
        }
    }

    /// returns the valid ID of the event
    #[must_use]
    pub const fn event_id(&self) -> &ValidEventID {
        &self.event_id
    }

    /// returns the specific data to be acted on
    #[must_use]
    pub const fn data(&self) -> &Event {
        &self.data
    }
}

impl PartialOrd for EventWrapper {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.time.cmp(&other.time))
    }
}

impl Ord for EventWrapper {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.time.cmp(&other.time)
    }
}

impl EventKind for EventWrapper {
    type EventKindError = DomainError;

    fn time(&self) -> &time::OffsetDateTime {
        &self.time
    }
}

impl Unique for EventWrapper {
    type Valid = event::ValidEventID;

    fn id(&self) -> &Self::Valid {
        &self.event_id
    }

    fn into_id(self) -> Self::Valid {
        self.event_id
    }
}

/// Event hold all the events that could happened to a `RedMaple`
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub enum Event {
    /// Event: An [`Entry`] was created.
    MapleCreated(Maple),

    /// Event: An already existing [`Entry`] was updated to a new version.
    MapleBodyUpdated(ValidMapleID, Body),
}

/// A thin wrapper around [`ID`] that validates that the [`ID`] is coming from an [`Journey`]
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ValidJourneyID(ID);

impl ValidID for ValidJourneyID {
    /// exposes the inner [`ID`] of the [`Journey`]
    #[must_use]
    fn inner(&self) -> &ID {
        &self.0
    }

    /// exposes the inner [`ID`] of the [`Journey`]
    fn into_inner(self) -> ID {
        self.0
    }
}

impl Unique for Journey {
    type Valid = ValidJourneyID;

    fn id(&self) -> &Self::Valid {
        &self.id
    }

    fn into_id(self) -> Self::Valid {
        self.id
    }
}

/// [`Journey`] is the holder of meta information for journeys
#[derive(Clone, Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct Journey {
    /// The unique [`ID`] of certain [`Journey`].
    id: ValidJourneyID,

    /// The time it was created.
    time_created: time::OffsetDateTime,
}

impl Journey {
    /// new creates a new instance of [`Journey`]
    #[must_use]
    pub const fn new(id: ID, time_created: time::OffsetDateTime) -> Self {
        Self {
            id: ValidJourneyID(id),
            time_created,
        }
    }
}

// /// [`ObjectType`] specifies the type of object
// #[derive(Clone, Debug, PartialEq, Eq)]
// pub enum ObjectType {
//     /// an object that is held in this [`Redmaple`]
//     Internal,
//     /// an object that is held in other [`Redmaple`]
//     External,
//     ///  n object that points to an specific time
//     Time,
// }

/// Errors that are resulted from functions  and emthods inside [`journey`]
#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub enum DomainError {
    /// For when a text field should contain 1 or more characters
    #[error("Text Cannot have 0 length")]
    TextCannotBeEmpty,
}
