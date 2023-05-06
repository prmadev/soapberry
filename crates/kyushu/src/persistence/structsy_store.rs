//! Abstraction layer for storing data using [`structsy`]

////////////////////////////////////
// imports
////////////////////////////////////
pub mod events;
pub mod queries;

use std::path::PathBuf;

use structsy::{Structsy, StructsyError};

////////////////////////////////////
// main logic
///////////////////////////////////

/// [`DBFile`] is holder for a file that is supposed to be existing
#[derive(Debug, Clone)]
pub struct DBFile<State = NotKnown>(PathBuf, std::marker::PhantomData<State>);

/// this is a zero sized struct holding state of [`DBFile`]
/// for when the **file exists**
#[derive(Debug, Clone)]
pub struct Existing;

/// this is a zero sized struct holding state of [`DBFile`]
/// for when the file does **not exist**
#[derive(Debug, Clone)]
pub struct NotExisting;

/// this is a zero sized struct holding state of [`DBFile`]
/// for when the file is **not know to exist or not**
#[derive(Debug, Clone)]
pub struct NotKnown;

impl<State> DBFile<State> {
    /// returns the inner pathbuf
    #[must_use]
    pub const fn inner(&self) -> &PathBuf {
        &self.0
    }

    /// returns the inner pathbuf, consuming it self
    #[allow(clippy::missing_const_for_fn)] // clippy thinks it can be const, while it cannot
    #[must_use]
    pub fn into_inner(self) -> PathBuf {
        self.0
    }
}

impl From<PathBuf> for DBFile {
    fn from(file: PathBuf) -> Self {
        Self(file, std::marker::PhantomData::<NotKnown>)
    }
}

/// defines a state for the existence of db file
#[derive(Debug, Clone)]
pub enum KnownDBFile {
    /// if the dbfile is existing
    Existing(DBFile<Existing>),
    /// if the dbfile is non-exiting
    NotExisting(DBFile<NotExisting>),
}

impl TryFrom<DBFile<NotKnown>> for KnownDBFile {
    type Error = DBFileError;

    fn try_from(value: DBFile<NotKnown>) -> Result<Self, Self::Error> {
        let file_path = value.into_inner();
        if !file_path.try_exists()? {
            return Ok(Self::NotExisting(DBFile(
                file_path,
                std::marker::PhantomData::<NotExisting>,
            )));
        };
        Ok(Self::Existing(DBFile(
            file_path,
            std::marker::PhantomData::<Existing>,
        )))
    }
}

//
// into structsy
//

// from [`Existing`], so it reads the file
impl TryFrom<DBFile<Existing>> for Structsy {
    // TODO: decide if  the error should be the raw structsy output
    type Error = StructsyError;

    fn try_from(file: DBFile<Existing>) -> Result<Self, Self::Error> {
        Self::open(file.inner())
    }
}

// from [`NotExisting`], so it creates the file
impl TryFrom<DBFile<NotExisting>> for Structsy {
    // TODO: decide if  the error should be the raw structsy output
    type Error = StructsyError;

    fn try_from(file: DBFile<NotExisting>) -> Result<Self, Self::Error> {
        Self::open(file.inner())
    }
}

////////////////////////////////////
// Errors
////////////////////////////////////

/// errors that may returns when trying to check for the existence of database file
#[derive(thiserror::Error, Debug)]
pub enum DBFileError {
    /// this happens when we are trying to validate the state of the db file
    #[error("Could not check the status of db file {0}")]
    FailedFileStatusCheck(#[from] std::io::Error),
}
