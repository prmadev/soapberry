//! it hold the logic of a contemplating journal.
//! it is difference from a zettelkasten-style journaling.
//! in that each node entry has a time associated with it.
//! and forms named relation ships.
//! these relationships form journeys

use std::time::SystemTime;

use getset_scoped::Getters;
use redmaple::id::ID;

/// Event hold all the events that could happened to a `RedMaple`
#[derive(Debug, Clone)]
pub enum Journal {
    /// Event: An [`Entry`] was created.
    EntryCreated(Entry),

    /// Event: An already existing [`Entry`] was **added** to an already existing [`Journey`].
    EntryAddedToJourney(ValidEntryID, ValidJourneyID),

    /// Event: An already existing [`Entry`] was **removed** from an already existing [`Journey`].
    EntryRemovedFromJourney(ValidEntryID, ValidJourneyID),

    /// Event: An already existing [`Entry`] was updated to a new version.
    EntryUpdated(ValidEntryID, Entry),

    /// Event: An already existing [`Entry`] added a new [`Link`].
    EntryLinked(Link),

    /// Event: A new [`Journey`] was created.
    JourneyCreated(Journey),

    /// Event: An already existing [`Journey`] Got a new [`Title`].
    JourneyRenamed(ValidJourneyID, Title),

    /// Event: An already existing [`Journey`] was deleted.
    JourneyDeleted(ValidJourneyID),
}

/// `Body` is a wrapper around simple [`String`] to ensure that the text alway follows the domain rules
#[derive(Clone, Debug)]
#[repr(transparent)]
pub struct Body(String);

impl Body {
    /// `build` checks if the the domain rules are being followed
    ///
    ///
    /// # Errors
    ///
    /// * [`JourneyError::TextCannotBeEmpty`] can be returned in-case of empty [`String`].
    pub fn build(text: String) -> Result<Self, DomainError> {
        if text.is_empty() {
            return Err(DomainError::TextCannotBeEmpty);
        };

        Ok(Self(text))
    }

    /// the inner string of [`Body`]
    #[must_use]
    pub const fn inner(&self) -> &String {
        &self.0
    }
}

/// Errors that are resulted from functions  and emthods inside [`journey`]
#[derive(thiserror::Error, Debug, Clone)]
pub enum DomainError {
    /// For when a text field should contain 1 or more characters
    #[error("Text Cannot have 0 length")]
    TextCannotBeEmpty,
}

/// [`Title`] is similar to [`Body`] in that it is a wrapper around simple [`String`] to
/// ensure that the text alway follows the domain rules.
#[derive(Clone, Debug)]
#[repr(transparent)]
pub struct Title(String);

impl Title {
    /// `build` checks if the the domain rules are being followed
    ///
    ///
    /// # Errors
    ///
    /// * [`JourneyError::TextCannotBeEmpty`] can be returned in-case of empty [`String`].
    pub fn build(text: String) -> Result<Self, DomainError> {
        if text.is_empty() {
            return Err(DomainError::TextCannotBeEmpty);
        };
        Ok(Self(text))
    }

    /// The inner string  of [`Title`]
    #[must_use]
    pub const fn inner(&self) -> &String {
        &self.0
    }
}

/// [`Entry`] contains information related to an specific user entry
///
///
#[derive(Clone, Debug, Getters)]
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
    journeys: Vec<ID>,
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
        journeys: Vec<ID>,
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
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct ValidEntryID(ID);

impl ValidEntryID {
    /// exposes the inner [`ID`] of the [`Entry`]
    #[must_use]
    pub const fn inner(&self) -> &ID {
        &self.0
    }
}

/// A thin wrapper around [`ID`] that validates that the [`ID`] is coming from an [`Journey`]
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct ValidJourneyID(ID);

impl ValidJourneyID {
    /// exposes the inner [`ID`] of the [`Journey`]
    #[must_use]
    pub const fn inner(&self) -> &ID {
        &self.0
    }
}

/// A thin wrapper around [`ID`] that validates that the [`ID`] is coming from an [`Link`]
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct ValidLinkID(ID);

impl ValidLinkID {
    /// exposes the inner [`ID`] of the [`Link`]
    #[must_use]
    pub const fn inner(&self) -> &ID {
        &self.0
    }
}

/// [`Link`] is the holder of information between two valid objects
#[derive(Clone, Debug, Getters)]
pub struct Link {
    /// The unique [`ID`] of certain [`Link`].
    #[getset(get = "pub")]
    id: ValidLinkID,

    /// The time it was created.
    #[getset(get = "pub")]
    time_created: SystemTime,

    /// The unique [`ID`] of certain the [`Entry`] which the [`Link`] is started from.  
    #[getset(get = "pub")]
    from_id: ID,

    /// The unique [`ID`] of certain the [`Entry`] which the [`Link`] is pointing to.  
    #[getset(get = "pub")]
    to_id: ID,

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
        from_id: ID,
        to_id: ID,
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

/// [`Journey`] is the holder of meta information for journeys
#[derive(Clone, Debug, Getters)]
pub struct Journey {
    /// The unique [`ID`] of certain [`Journey`].
    #[getset(get = "pub")]
    id: ValidJourneyID,

    /// The time it was created.
    #[getset(get = "pub")]
    time_created: SystemTime,

    /// [`Title`] of the [`Entry`]
    #[getset(get = "pub")]
    title: Title,
}

impl Journey {
    /// new creates a new instance of [`Journey`]
    #[must_use]
    pub const fn new(id: ID, time_created: SystemTime, title: Title) -> Self {
        Self {
            id: ValidJourneyID(id),
            time_created,
            title,
        }
    }
}

/// [`ObjectType`] specifies the type of object
#[derive(Clone, Debug)]
pub enum ObjectType {
    /// an object that is held in this [`Redmaple`]  
    Internal,
    /// an object that is held in other [`Redmaple`]  
    External,
    ///  n object that points to an specific time
    Time,
}
