//! file db
use std::{collections::HashMap, io::Write, path::PathBuf, time::SystemTimeError};

use redmaple::{
    id::{IDGiver, ID},
    EventRepo, RedMaple,
};
use walkdir::WalkDir;
use whirlybird::journey::JournalEventWrapper;

#[derive(Debug, Clone)]
pub struct FileDB {
    events: std::collections::HashMap<ID, RedMaple<JournalEventWrapper>>,
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
                    .map(|c| serde_json::from_slice::<RedMaple<JournalEventWrapper>>(&c))
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
        let events: HashMap<ID, RedMaple<JournalEventWrapper>> = files
            .into_iter()
            .filter_map(Result::ok)
            .map(|f| (f.id().inner().to_owned(), f))
            .collect();

        Ok(Self {
            events,
            path: path_to_rep,
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RebuildError {
    #[error("the given path does not exist")]
    GivenPathDoesNotExit,

    #[error("could not read the directory")]
    CouldNotReadTheDirectory(std::io::Error),

    #[error("got error processing files {0:?}")]
    CouldNotProcessesAFile(Vec<std::io::Error>),

    #[error("got error deserialize a files {0:?}")]
    CouldNotDeserializeAFile(Vec<serde_json::Error>),
}

impl EventRepo for FileDB {
    type Item = JournalEventWrapper;

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
            .join(format!("{}.json", item.id().inner().inner()));

        let s = serde_json::to_string_pretty(&item)
            .map_err(EventRepoError::CouldNotSerialize)?
            .into_bytes();

        std::fs::File::create(file_path)
            .map_err(EventRepoError::CouldNotCreateNewFile)?
            .write_all(&s)
            .map_err(EventRepoError::CouldNotWriteIntoFile)?;

        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum EventRepoError {
    #[error("could not find item")]
    CouldNotFindTheEventWithThatID,

    #[error("couldn not serialize: {0}")]
    CouldNotSerialize(#[from] serde_json::Error),

    #[error("couldn not create path for new file: {0}")]
    CouldNotCreatePathForNewFile(#[from] SystemTimeError),

    #[error("file already exist.")]
    FileAlreadyExist,

    #[error("could not create new file: {0}")]
    CouldNotCreateNewFile(std::io::Error),

    #[error("could write data into file: {0}")]
    CouldNotWriteIntoFile(std::io::Error),
}
