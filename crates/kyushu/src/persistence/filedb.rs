//! This module provides the `FileDB` implementation, which serves as a plaintext persistence layer for redmaple.

use once_cell::sync::OnceCell;
use redmaple::{
    id::{ValidID, ID},
    FrostElf, RedMaple,
};
use std::{collections::HashMap, fs::read_dir, path::PathBuf};
use whirlybird::journey::{EventWrapper, IDGetterError, ValidMapleID};

/// Represents the `FileDB` implementation, which is a file-based local `RedMaple` repository.
#[derive(Debug, Clone)]
pub struct FileDB {
    events: OnceCell<HashMap<ID, RedMaple<EventWrapper>>>,
    path_to_repo: PathBuf,
}

impl TryFrom<PathBuf> for FileDB {
    type Error = FrostElfError;

    fn try_from(path_to_repo: PathBuf) -> Result<Self, Self::Error> {
        // IO impurity
        if !path_to_repo.exists() {
            return Err(FrostElfError::GivenPathDoesNotExist);
        }

        Ok(Self {
            events: OnceCell::new(),
            path_to_repo,
        })
    }
}

impl FrostElf for FileDB {
    type Item = EventWrapper;

    type EventError = FrostElfError;

    fn redmaple_matching_id(&self, id: &ID) -> Result<&RedMaple<Self::Item>, Self::EventError> {
        self.all_redmaples_as_map()?
            .get(id)
            .ok_or(FrostElfError::FailedToFindTheEventWithThatID)
    }

    fn redmaple_similar_id(&self, id: &ID) -> Result<&RedMaple<Self::Item>, Self::EventError> {
        let founded_ids: Vec<_> = self
            .all_redmaples_as_map()?
            .keys()
            .filter(|x| x.to_string().contains(&id.to_string()))
            .collect();

        if founded_ids.len() != 1 {
            return Err(FrostElfError::FailedToFindASingleMatchingItem(
                founded_ids
                    .into_iter()
                    .map(std::clone::Clone::clone)
                    .collect(),
            ));
        }

        let founded_id = founded_ids
            .first()
            .ok_or(FrostElfError::FailedToFindTheEventWithThatID)?;

        self.all_redmaples_as_map()?
            .get(founded_id)
            .ok_or(FrostElfError::FailedToFindTheEventWithThatID)
    }

    fn all_redmaples_as_map(&self) -> Result<&HashMap<ID, RedMaple<Self::Item>>, Self::EventError> {
        self.events.get_or_try_init(|| {
            // IO impurity
            Ok(read_dir(&self.path_to_repo) // create a directory reader
                //then mapping the error of directory reading
                .map_err(FrostElfError::CouldNotReadTheDirectory)?
                // filtering all the items that are not ok
                .filter_map(Result::ok) // filter those that are ok
                // we only use json files
                .filter(|direntry| direntry.path().extension().map_or(false, |e| e == "json"))
                // creating a redmaple from each file
                // IO impurity
                .map(|value| {
                    std::fs::read(value.path())
                        .map_err(FrostElfError::FileReadFailed)
                        .map(|raw| {
                            let the_redmaple =
                                serde_json::from_slice::<RedMaple<EventWrapper>>(&raw)
                                    .map_err(FrostElfError::FailedToSerialize)?;
                            Ok((
                                ValidMapleID::try_from(&the_redmaple)
                                    .map_err(FrostElfError::FailedToGetID)?
                                    .inner().clone(),
                                the_redmaple,
                            ))
                        })?
                })
                // then filtering all those that are not redmaples
                .fold(
                    Ok(HashMap::<ID, RedMaple<EventWrapper>>::new()),
                    |acc: Result<_, FrostElfError>, item: Result<(ID, RedMaple<EventWrapper>), FrostElfError>| {
                        let the_item = item?;
                        let mut maple_map = acc?;
                        _ = maple_map.insert(the_item.0, the_item.1);
                        Ok(maple_map)
                    },
                )?)
        })
    }

    fn save(
        &self,
        item: RedMaple<Self::Item>,
        should_overwrite: bool,
    ) -> Result<(), Self::EventError> {
        let file_path = self
            .path_to_repo
            .join(format!("{}.json", ValidMapleID::try_from(&item)?.inner()));

        // IO impurity
        if file_path.exists() && !should_overwrite {
            return Err(FrostElfError::FileExists(file_path));
        }

        let s = serde_json::to_string_pretty(&item)
            .map_err(FrostElfError::FailedToSerialize)?
            .into_bytes();

        // IO impurity
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

    /// Multiple given file already exists.
    #[error("redmaple file already exists {0}")]
    FileExists(PathBuf),

    /// The file content could not be read.
    #[error("failed to read the content of the file: {0}")]
    FileReadFailed(std::io::Error),

    /// if a path is not given
    #[error("the given path does not exist")]
    GivenPathDoesNotExist,

    /// indicates that the file at the given address does not exist.
    /// this should not happen.
    #[error("could not read the directory")]
    CouldNotReadTheDirectory(std::io::Error),
}
