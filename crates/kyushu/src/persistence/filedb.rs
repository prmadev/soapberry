//! file db
use std::{collections::HashMap, path::PathBuf};

use redmaple::{id::ID, EventRepo, RedMaple};
use whirlybird::journey::{EventWrapper, IDGetterError, ValidMapleID};

/// [`FileDB`] is a the implementation of file based local [`RedMapleRepo`]
#[derive(Debug, Clone)]
pub struct FileDB {
    events: std::collections::HashMap<ID, RedMaple<EventWrapper>>,
    path: PathBuf,
}

impl TryFrom<PathBuf> for FileDB {
    type Error = RebuildError;

    fn try_from(path_to_rep: PathBuf) -> Result<Self, Self::Error> {
        if !path_to_rep.exists() {
            return Err(RebuildError::GivenPathDoesNotExit);
        }

        // read the directory for files
        let events = std::fs::read_dir(&path_to_rep)
            .map_err(RebuildError::CouldNotReadTheDirectory)?
            .filter_map(Result::ok) // filter those that are ok
            .map(redmaple_from_file)
            .filter_map(|x| match x {
                Ok(o) => match o {
                    Some(rm) => match ValidMapleID::try_from(&rm) {
                        Ok(i) => Some(Ok((i.inner().clone(), rm))),
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

/// errors that can can arsie whene rebuilding information from files
#[derive(Debug, thiserror::Error)]
pub enum RebuildError {
    /// if a path is not given
    #[error("the given path does not exist")]
    GivenPathDoesNotExit,

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
    COuldNOtReadDirectory(std::io::Error),
}

impl EventRepo for FileDB {
    type Item = EventWrapper;

    type EventError = EventRepoError;

    fn redmaple_matching_id(&self, id: &ID) -> Result<&RedMaple<Self::Item>, Self::EventError> {
        self.events
            .get(id)
            .ok_or(EventRepoError::CouldNotFindTheEventWithThatID)
    }

    fn redmaple_similar_id(&self, id: &ID) -> Result<&RedMaple<Self::Item>, Self::EventError> {
        let sid = id.to_string();
        let finding: Vec<_> = self
            .events
            .keys()
            .filter(|x| x.to_string().contains(&sid))
            .collect();

        if finding.len() != 1 {
            return Err(EventRepoError::MultipleItemsFound(
                finding.into_iter().map(std::clone::Clone::clone).collect(),
            ));
        }
        let idfounded = finding
            .first()
            .ok_or(EventRepoError::CouldNotFindTheEventWithThatID)?;

        self.events
            .get(idfounded)
            .ok_or(EventRepoError::CouldNotFindTheEventWithThatID)
    }

    fn all_events(&self) -> Result<&HashMap<ID, RedMaple<Self::Item>>, Self::EventError> {
        Ok(&self.events)
    }

    fn save(&self, item: RedMaple<Self::Item>) -> Result<(), Self::EventError> {
        let file_path = self
            .path
            .join(format!("{}.json", ValidMapleID::try_from(&item)?.inner()));

        let s = serde_json::to_string_pretty(&item)
            .map_err(EventRepoError::CouldNotSerialize)?
            .into_bytes();

        std::fs::write(file_path, s).map_err(EventRepoError::CouldNotWriteIntoFile)
    }
}

/// Errors related to the implementation of [`EventRepo`] trait for the [`FileDB`]
#[derive(thiserror::Error, Debug)]
pub enum EventRepoError {
    /// Could not find a particular item
    #[error("could not find item")]
    CouldNotFindTheEventWithThatID,

    /// could not serialize a given data
    #[error("couldn not serialize: {0}")]
    CouldNotSerialize(#[from] serde_json::Error),

    /// for some reason the file could not be created
    #[error("could not create new file: {0}")]
    CouldNotCreateNewFile(std::io::Error),

    /// for some reason the file could not be write into
    #[error("could write data into file: {0}")]
    CouldNotWriteIntoFile(std::io::Error),

    /// could not get id from event repo
    #[error("could not get event redmaple id: {0}")]
    IDGettingFailed(#[from] IDGetterError),

    /// could not get id from event repo
    #[error("multiple items found: {0:?}")]
    MultipleItemsFound(Vec<ID>),
}

#[allow(clippy::needless_pass_by_value)] // the value is not being used any further in the original function
fn redmaple_from_file(
    value: std::fs::DirEntry,
) -> Result<Option<RedMaple<EventWrapper>>, FromFileError> {
    if !value.path().extension().map_or(false, |e| e == "json") {
        return Ok(None);
    }

    Ok(Some(serde_json::from_slice::<RedMaple<EventWrapper>>(
        &std::fs::read(value.path()).map_err(FromFileError::FileNotReadable)?,
    )?))
}

/// failiure in converting `DirEntry` to to `RedMaple`
#[derive(thiserror::Error, Debug)]
pub enum FromFileError {
    /// Not a Json file
    #[error("could not read the content of the file: {0}")]
    FileNotReadable(std::io::Error),

    /// could not serialize a given data
    #[error("couldn not serialize: {0}")]
    CouldNotSerialize(#[from] serde_json::Error),
}
