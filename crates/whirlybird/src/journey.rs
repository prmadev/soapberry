//! Contemplating Journal
//!
//! This module encapsulates the intricate workings of a Contemplating Journal, a captivating departure from conventional journaling approaches. Prepare to embark on a remarkable cognitive odyssey guided by the profound wisdom of Robert Sapolsky.
//!
//! In this extraordinary journaling paradigm, we diverge from the well-trodden path of zettelkasten-style journaling. Instead, we embrace a transformative methodology where each journal entry is imbued with the essence of time itself. Every node becomes a vessel for contemplation, forever entwined with the temporal fabric of our existence.
//!
//! Unlike its counterparts, this journaling system forges profound connections through named relationships. Like synapses sparking in a neural network, ideas form intricate webs of interdependence, rendering a tapestry of thought that transcends the boundaries of conventional wisdom.
//!
//! These interwoven relationships birth a multitude of intellectual journeys, weaving narratives that unravel the mysteries of our consciousness. Through the profound integration of time, contemplation, and relationship, we unravel the secrets of our minds and unearth the transformative power of introspection.
//!
//! Embark on this extraordinary expedition of intellectual exploration. Embrace the journal's capacity to ignite the flames of curiosity and propel you towards novel insights and profound self-discovery. Be prepared to immerse yourself in a realm where time bends, relationships blossom, and thought blossoms in its purest form.
//!
//! As we delve into the depths of this Contemplating Journal, guided by the spirit of Robert Sapolsky's illuminating wisdom, let us unlock the boundless potential of our minds and unravel the enigma of our existence.
//!
//! Now, let your thoughts soar on the wings of contemplation, and embark on this awe-inspiring quest of self-discovery and intellectual enlightenment.
//!
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

/// [`EventWrapper`] holds the meta data for [`Journal`] event
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
                Event::MapleBodyUpdated(_, _) | Event::LinkAdded(_) | Event::Dislinked(_) => ac,
            })
            .ok_or(IDGetterError::NoEventsFound)
    }
}

/// An error that occurs when attempting to retrieve a `ValidMapleID` from a `RedMaple` but no events are found.
#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub enum IDGetterError {
    /// Indicates a lack of events in the RedMaple, leaving us bereft of meaningful identification.
    #[error("The Great Void of RedMaple: No Events Found")]
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
    type EventKindError = EventKindErrorImplementation;

    fn time(&self) -> &time::OffsetDateTime {
        &self.time
    }
}

/// Empty Error holder
#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub enum EventKindErrorImplementation {}

impl Unique for EventWrapper {
    type Valid = event::ValidEventID;

    fn id(&self) -> &Self::Valid {
        &self.event_id
    }

    fn into_id(self) -> Self::Valid {
        self.event_id
    }
}

/// Event encompasses all possible events that can occur within a `RedMaple`.
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub enum Event {
    /// Event: Signifies the creation of a new [`Maple`] in the `RedMaple`.
    MapleCreated(Maple),

    /// Event: Denotes the updating of an existing [`Maple`] in the `RedMaple` to a new version.
    MapleBodyUpdated(ValidMapleID, Body),

    /// Event: Represents the addition of a link between multiple [`Maple`]s.
    LinkAdded((ValidMapleID, String, ID)),

    /// Event: Represents the removal of a link from [`Maple`]
    Dislinked(ValidLinkID),
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

/// `Journey` is the keeper of essential meta-information for journeys.
#[derive(Clone, Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct Journey {
    /// The unique identifier of this particular [`Journey`].
    id: ValidJourneyID,

    /// The timestamp when this [`Journey`] was created.
    time_created: time::OffsetDateTime,
}

impl Journey {
    /// Creates a new instance of [`Journey`] with the specified ID and creation time.
    #[must_use]
    pub const fn new(id: ID, time_created: time::OffsetDateTime) -> Self {
        Self {
            id: ValidJourneyID(id),
            time_created,
        }
    }
}
