//! persistence holds the logic for the persisting layer of the application

use std::{
    io::Write,
    path::PathBuf,
    time::{SystemTimeError, UNIX_EPOCH},
};

use redmaple::{event_group::EventGroup, id::ID, EventRepo};
use walkdir::WalkDir;
use whirlybird::journey::JournalEvent;

#[derive(Debug, Clone)]
pub struct FileRepo {
    events: redmaple::RedMaple<JournalEvent>,
    path: PathBuf,
}

impl TryFrom<PathBuf> for FileRepo {
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
                let status = match ext {
                    Some(e) if e == "json" => Some(f),
                    _ => None,
                };
                status
            }) // find out which ones are json
            .map(|f| std::fs::read(f.path()).map(|c| serde_json::from_slice::<JournalEvent>(&c))) // read them and and turn them into journl events
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
        let je: Vec<_> = files.into_iter().filter_map(Result::ok).collect();

        let events = redmaple::RedMaple::new(ID::new(uuid::Uuid::nil()), je);

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

impl EventRepo for FileRepo {
    type Item = JournalEvent;

    type EventError = EventRepoError;

    fn events_matching_id(&self, id: &redmaple::id::ID) -> Result<&Self::Item, Self::EventError> {
        self.events
            .events()
            .iter()
            .find(|ev| ev.event_id().inner().eq(id))
            .ok_or(EventRepoError::CouldNotFindTheEventWithThatID)
    }

    fn all_events(&self) -> Result<&Vec<Self::Item>, Self::EventError> {
        Ok(&self.events.events())
    }

    fn append(&self, item: Self::Item) -> Result<(), Self::EventError> {
        let file_path = self.path.join(format!(
            "{}.json",
            item.time().duration_since(UNIX_EPOCH)?.as_nanos()
        ));
        if std::path::Path::exists(&file_path) {
            return Err(EventRepoError::FileAlreadyExist);
        }

        let mut f =
            std::fs::File::create(&file_path).map_err(EventRepoError::CouldNotCreateNewFile)?;

        let s = serde_json::to_string_pretty(&item)
            .map_err(EventRepoError::CouldNotSerialize)?
            .into_bytes();

        f.write_all(&s)
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
