//! it hold the logic of a contemplating journal.
//! it is difference from a zettelkasten-style journaling.
//! in that each node entry has a time associated with it.
//! and forms named relation ships.
//! these relationships form journeys

#![allow(missing_docs, unused)]
use std::time::SystemTime;

use redmaple::id::ID;

/// Event hold all the events that could happened to a `RedMaple`
#[derive(Debug, Clone)]
pub enum Journal {
    EntryCreated(Entry),
    EntryAddedToJourney(ID, ID),
    EntryRemovedFromJourney(ID, ID),
    EntryUpdated(ID, Entry),
    EntryLinked(Link),
}

#[derive(Clone, Debug)]
#[repr(transparent)]
pub struct Body {
    text: String,
}

#[derive(Clone, Debug)]
#[repr(transparent)]
pub struct Title {
    text: String,
}

#[derive(Clone, Debug)]
pub struct Entry {
    id: ID,
    time_created: SystemTime,
    title: Option<Title>,
    body: Option<Body>,
    links: Vec<Link>,
    journeys: Vec<ID>,
}

#[derive(Clone, Debug)]
pub struct Link {
    id: ID,
    time_created: SystemTime,
    from_id: ID,
    to_id: ID,
    title: Title,
    body: Body,
}

#[derive(Clone, Debug)]
pub enum ObjectType {
    Internal,
    External,
    Time,
}
