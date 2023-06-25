//! file db
use std::{collections::HashMap, path::PathBuf};

use redmaple::{
    id::{IDGiver, ID},
    EventRepo, RedMaple,
};
use walkdir::WalkDir;
use whirlybird::journey::EventWrapper;

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
        let (files, errs): (Vec<_>, Vec<_>) = WalkDir::new(&path_to_rep)
            .into_iter()
            .filter_map(Result::ok) // filter those that are ok
            .filter_map(|f| {
                let ext = f.path().extension();
                match ext {
                    Some(e) if e == "json" => Some(f),
                    _ => None,
                }
            }) // find out which ones are json
            .map(|f| {
                std::fs::read(f.path())
                    .map(|c| serde_json::from_slice::<RedMaple<EventWrapper>>(&c))
            }) // read them and and turn them into journl events
            .partition(Result::is_ok);

        // pass 1 of errors: errors of not being able to read files
        if !errs.is_empty() {
            let errs: Vec<_> = errs.into_iter().filter_map(Result::err).collect();
            return Err(RebuildError::CouldNotProcessesAFile(errs));
        };

        let (files, errs): (Vec<_>, Vec<_>) = files.into_iter().partition(Result::is_ok);

        // pass 2 of errors
        if !errs.is_empty() {
            let errs: Vec<_> = errs.into_iter().filter_map(Result::err).collect();
            return Err(RebuildError::CouldNotProcessesAFile(errs));
        };

        let files: Vec<_> = files.into_iter().filter_map(Result::ok).collect();

        // pass 3 of errors
        let (files, errs): (Vec<_>, Vec<_>) = files.into_iter().partition(Result::is_ok);

        if !errs.is_empty() {
            let errs: Vec<_> = errs.into_iter().filter_map(Result::err).collect();
            return Err(RebuildError::CouldNotDeserializeAFile(errs));
        };

        // the results
        let events: HashMap<ID, RedMaple<EventWrapper>> = files
            .into_iter()
            .filter_map(Result::ok)
            .map(|f| (f.id().inner().clone(), f))
            .collect();

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
    CouldNotProcessesAFile(Vec<std::io::Error>),

    /// error that happen as a result of inconsistent formating of json files
    #[error("got error deserialize a files {0:?}")]
    CouldNotDeserializeAFile(Vec<serde_json::Error>),
}

impl EventRepo for FileDB {
    type Item = EventWrapper;

    type EventError = EventRepoError;

    fn redmaple_matching_id(&self, id: &ID) -> Result<&RedMaple<Self::Item>, Self::EventError> {
        self.events
            .get(id)
            .ok_or(EventRepoError::CouldNotFindTheEventWithThatID)
    }

    fn all_events(&self) -> Result<&HashMap<ID, RedMaple<Self::Item>>, Self::EventError> {
        Ok(&self.events)
    }

    fn save(&self, item: RedMaple<Self::Item>) -> Result<(), Self::EventError> {
        let file_path = self
            .path
            .join(format!("{}.json", item.id().inner().inner(),));

        let s = serde_json::to_string_pretty(&item)
            .map_err(EventRepoError::CouldNotSerialize)?
            .into_bytes();

        std::fs::write(file_path, &s).map_err(EventRepoError::CouldNotWriteIntoFile)
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
}
