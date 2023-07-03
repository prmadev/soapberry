//! This module provides the `FileDB` implementation, which serves as a plaintext persistence layer for redmaple.
use std::{collections::HashMap, path::PathBuf};

use redmaple::{
    id::{ValidID, ID},
    FrostElf, RedMaple,
};
use whirlybird::journey::{EventWrapper, IDGetterError, ValidMapleID};

/// Represents the `FileDB` implementation, which is a file-based local `RedMaple` repository.
#[derive(Debug, Clone)]
pub struct FileDB {
    events: std::collections::HashMap<ID, RedMaple<EventWrapper>>,
    path: PathBuf,
}

impl TryFrom<PathBuf> for FileDB {
    type Error = RebuildError;

    fn try_from(path_to_rep: PathBuf) -> Result<Self, Self::Error> {
        if !path_to_rep.exists() {
            return Err(RebuildError::GivenPathDoesNotExist);
        }

        // Read the directory for files
        let events = std::fs::read_dir(&path_to_rep) // create a directory reader
            //then mapping the error of directory reading
            .map_err(RebuildError::CouldNotReadTheDirectory)?
            // filtering all the items that are not ok
            .filter_map(Result::ok) // filter those that are ok
            // creating a redmaple from each file
            .map(redmaple_from_file)
            // then filtering all those that are not redmaples
            .filter_map(|x| match x {
                Ok(o) => match o {
                    Some(rm) => match ValidMapleID::try_from(&rm) {
                        Ok(i) => Some(Ok((i.inner().clone(), rm))),
                        // then for invalid redmaples
                        Err(ers) => Some(Err(RebuildError::CouldNotGetTheIDRedmaple(ers))),
                    },
                    None => None,
                },
                Err(e) => Some(Err(RebuildError::CouldNotProcessesAFile(e))),
            })
            .fold(
                Ok(HashMap::<ID, RedMaple<EventWrapper>>::new()),
                |acc: Result<_, Self::Error>, item| {
                    let i = item?;
                    let mut res = acc?;
                    _ = res.insert(i.0, i.1);
                    Ok(res)
                },
            )?;

        Ok(Self {
            events,
            path: path_to_rep,
        })
    }
}

/// Represents errors that can occur during rebuilding information from files.
#[derive(Debug, thiserror::Error)]
pub enum RebuildError {
    /// if a path is not given
    #[error("the given path does not exist")]
    GivenPathDoesNotExist,

    /// indicates that the file at the given address does not exist.
    /// this should not happen.
    #[error("could not read the directory")]
    CouldNotReadTheDirectory(std::io::Error),

    /// Error that happen when reading the files fail
    #[error("got error processing files {0:?}")]
    CouldNotProcessesAFile(FromFileError),

    /// ID of redmaple could not be read
    #[error("got error processing files {0:?}")]
    CouldNotGetTheIDRedmaple(IDGetterError),

    /// Could not read directory
    #[error("failed to read the directory")]
    CouldNotReadDirectory(std::io::Error),
}

impl FrostElf for FileDB {
    type Item = EventWrapper;

    type EventError = FrostElfError;

    fn redmaple_matching_id(&self, id: &ID) -> Result<&RedMaple<Self::Item>, Self::EventError> {
        self.events
            .get(id)
            .ok_or(FrostElfError::FailedToFindTheEventWithThatID)
    }

    fn redmaple_similar_id(&self, id: &ID) -> Result<&RedMaple<Self::Item>, Self::EventError> {
        let sid = id.to_string();
        let finding: Vec<_> = self
            .events
            .keys()
            .filter(|x| x.to_string().contains(&sid))
            .collect();

        if finding.len() != 1 {
            return Err(FrostElfError::FailedToFindASingleMatchingItem(
                finding.into_iter().map(std::clone::Clone::clone).collect(),
            ));
        }
        let idfounded = finding
            .first()
            .ok_or(FrostElfError::FailedToFindTheEventWithThatID)?;

        self.events
            .get(idfounded)
            .ok_or(FrostElfError::FailedToFindTheEventWithThatID)
    }

    fn all_events(&self) -> Result<&HashMap<ID, RedMaple<Self::Item>>, Self::EventError> {
        Ok(&self.events)
    }

    fn save(&self, item: RedMaple<Self::Item>) -> Result<(), Self::EventError> {
        let file_path = self
            .path
            .join(format!("{}.json", ValidMapleID::try_from(&item)?.inner()));

        let s = serde_json::to_string_pretty(&item)
            .map_err(FrostElfError::FailedToSerialize)?
            .into_bytes();

        std::fs::write(file_path, s).map_err(FrostElfError::FailedToWriteIntoFile)
    }
}

/// Errors related to the implementation of [`EventRepo`] trait for the [`FileDB`]
#[derive(thiserror::Error, Debug)]
pub enum FrostElfError {
    /// Failed to find the requested item.
    #[error("could not find item")]
    FailedToFindTheEventWithThatID,

    /// Failed to serialize the given data.
    #[error("couldn not serialize: {0}")]
    FailedToSerialize(#[from] serde_json::Error),

    /// Failed to create a new file.
    #[error("could not create new file: {0}")]
    FailedToCreateNewFile(std::io::Error),

    /// Failed to write data into the file.
    #[error("could write data into file: {0}")]
    FailedToWriteIntoFile(std::io::Error),

    /// Failed to retrieve the ID from the event repository.
    #[error("could not get event redmaple id: {0}")]
    FailedToGetID(#[from] IDGetterError),

    /// Multiple items with the same ID were found.
    #[error("multiple items found: {0:?}")]
    FailedToFindASingleMatchingItem(Vec<ID>),
}

#[allow(clippy::needless_pass_by_value)] // the value is not being used any further in the original function
fn redmaple_from_file(
    value: std::fs::DirEntry,
) -> Result<Option<RedMaple<EventWrapper>>, FromFileError> {
    if !value.path().extension().map_or(false, |e| e == "json") {
        return Ok(None);
    }

    Ok(Some(serde_json::from_slice::<RedMaple<EventWrapper>>(
        &std::fs::read(value.path()).map_err(FromFileError::FileReadFailed)?,
    )?))
}

/// Error that occurs during the conversion from `DirEntry` to `RedMaple`.
#[derive(thiserror::Error, Debug)]
pub enum FromFileError {
    /// The file content could not be read.
    #[error("failed to read the content of the file: {0}")]
    FileReadFailed(std::io::Error),

    /// Failed to serialize the given data.
    #[error("failed to serialize data: {0}")]
    SerializationFailed(#[from] serde_json::Error),
}
