//! This module provides the `FileDB` implementation, which serves as a plaintext persistence layer for redmaple.

use once_cell::sync::OnceCell;
use redmaple::{
    id::{ValidID, ID},
    BeeElf, CartographerElf, GardnerElf, RedMaple, SeekingElf, TrackerElf,
};
use std::{collections::HashMap, fs::read_dir, path::PathBuf};
use whirlybird::journey::{EventWrapper, IDGetterError, ValidMapleID};

// pub struct AnaTheWise() -> seek
// pub struct KamandTheWeird; -> gardner
// pub struct PegahTheCute(); -> cartograph
// pub struct ParastooThePassionate(); -> keeper
// pub struct IliyaTheKachal();
// pub struct AhmadTheFriend();

/// Selda holds a valid path for our forest
pub struct SeldaTheListener(PathBuf);

impl TryFrom<PathBuf> for SeldaTheListener {
    type Error = FrostElfError;

    fn try_from(path_to_forest: PathBuf) -> Result<Self, Self::Error> {
        // IO impurity
        if !path_to_forest.exists() {
            return Err(FrostElfError::GivenPathDoesNotExist(path_to_forest));
        }

        Ok(Self(path_to_forest))
    }
}

/// Amin plants new maples
pub struct AminTheSinger(SeldaTheListener);

impl BeeElf for AminTheSinger {
    type Item = EventWrapper;

    type EventError = FrostElfError;

    fn plant(&self, item: RedMaple<Self::Item>) -> Result<(), Self::EventError> {
        let file_path = self
            .0
             .0
            .join(format!("{}.json", ValidMapleID::try_from(&item)?.inner()));

        // IO impurity
        if file_path.exists() {
            return Err(FrostElfError::FileExists(file_path));
        }

        let the_song = serde_json::to_string_pretty(&item)
            .map_err(FrostElfError::FailedToSerialize)?
            .into_bytes();

        // IO impurity
        std::fs::write(file_path, the_song).map_err(FrostElfError::FailedToWriteIntoFile)
    }
}

impl From<SeldaTheListener> for AminTheSinger {
    fn from(selda: SeldaTheListener) -> Self {
        Self(selda)
    }
}

/// parastoo keeps a list of all Maples
pub struct ParastooTheKeeper {
    vines: Vec<(PathBuf, OnceCell<RedMaple<EventWrapper>>)>,
    the_path: SeldaTheListener,
}

impl TrackerElf for ParastooTheKeeper {
    type Item = EventWrapper;

    type EventError = FrostElfError;

    fn maples(&self) -> Result<Vec<&RedMaple<EventWrapper>>, FrostElfError> {
        self.vines
            .iter()
            .map(|(the_maples_path, maple)| {
                maple.get_or_try_init(|| {
                    std::fs::read(the_maples_path)
                        .map_err(FrostElfError::FileReadFailed)
                        .map(|raw| {
                            serde_json::from_slice::<RedMaple<EventWrapper>>(&raw)
                                .map_err(FrostElfError::FailedToSerialize)
                        })?
                })
            })
            .fold(
                Ok(vec![]),
                |acc: Result<Vec<&RedMaple<EventWrapper>>, FrostElfError>,
                 item: Result<&RedMaple<EventWrapper>, FrostElfError>| {
                    let the_item = item?;
                    let mut forest: Vec<&RedMaple<EventWrapper>> = acc?;
                    forest.push(the_item);
                    Ok(forest)
                },
            )
    }
}
impl CartographerElf for ParastooTheKeeper {
    type Item = EventWrapper;

    type EventError = FrostElfError;

    fn all_redmaples_as_map(&self) -> Result<HashMap<ID, &RedMaple<Self::Item>>, Self::EventError> {
        self.vines
            .iter()
            .map(|(the_maple_path, the_maple)| {
                the_maple.get_or_try_init(|| -> Result<RedMaple<EventWrapper>, FrostElfError> {
                    std::fs::read(the_maple_path)
                        .map_err(FrostElfError::FileReadFailed)
                        .map(|raw| {
                            serde_json::from_slice::<RedMaple<EventWrapper>>(&raw)
                                .map_err(FrostElfError::FailedToSerialize)
                        })?
                })
            })
            .fold(
                Ok(HashMap::new()),
                |acc: Result<HashMap<ID, &RedMaple<EventWrapper>>, FrostElfError>, item| {
                    let the_maple = item?;
                    let the_id = ValidMapleID::try_from(the_maple)?;
                    acc.map(|mut the_map| {
                        _ = the_map.insert(the_id.into_inner(), the_maple);
                        the_map
                    })
                },
            )
    }
}
impl SeekingElf for ParastooTheKeeper {
    type Item = EventWrapper;

    type EventError = FrostElfError;

    fn redmaple_matching_id(&self, id: &ID) -> Result<&RedMaple<Self::Item>, Self::EventError> {
        self.vines
            .iter()
            .map(|(the_maple_path, a_maple)| {
                let the_maple = a_maple.get_or_try_init(
                    || -> Result<RedMaple<EventWrapper>, FrostElfError> {
                        std::fs::read(the_maple_path)
                            .map_err(FrostElfError::FileReadFailed)
                            .map(|raw| {
                                serde_json::from_slice::<RedMaple<EventWrapper>>(&raw)
                                    .map_err(FrostElfError::FailedToSerialize)
                            })?
                    },
                )?;
                let the_maple_id =
                    ValidMapleID::try_from(the_maple).map_err(FrostElfError::FailedToGetID)?;
                Ok((the_maple_id, the_maple))
            })
            .find(|x| match x {
                Ok(maple_under_question) => maple_under_question.0.inner() == id,
                Err(_) => true, // in case of first error come back to me
            })
            .ok_or(FrostElfError::FailedToFindTheEventWithThatID)?
            .map(|x| x.1)
    }

    fn redmaple_similar_id(&self, id: &ID) -> Result<&RedMaple<Self::Item>, Self::EventError> {
        self.vines
            .iter()
            .map(|(the_maple_path, a_maple)| {
                let the_maple = a_maple.get_or_try_init(
                    || -> Result<RedMaple<EventWrapper>, FrostElfError> {
                        std::fs::read(the_maple_path)
                            .map_err(FrostElfError::FileReadFailed)
                            .map(|raw| {
                                serde_json::from_slice::<RedMaple<EventWrapper>>(&raw)
                                    .map_err(FrostElfError::FailedToSerialize)
                            })?
                    },
                )?;
                let the_maple_id =
                    ValidMapleID::try_from(the_maple).map_err(FrostElfError::FailedToGetID)?;
                Ok((the_maple_id, the_maple))
            })
            .find(|x| match x {
                Ok(maple_under_question) => maple_under_question
                    .0
                    .inner()
                    .inner()
                    .to_string()
                    .contains(&id.inner().to_string()),
                Err(_) => true, // in case of first error come back to me
            })
            .ok_or(FrostElfError::FailedToFindTheEventWithThatID)?
            .map(|x| x.1)
    }
}
impl GardnerElf for ParastooTheKeeper {
    type Item = EventWrapper;

    type EventError = FrostElfError;

    fn tend(&self, item: RedMaple<Self::Item>) -> Result<(), Self::EventError> {
        let file_path = self
            .the_path
            .0
            .join(format!("{}.json", ValidMapleID::try_from(&item)?.inner()));

        // IO impurity
        if !file_path.exists() {
            return Err(FrostElfError::FileDoesNotExists(file_path));
        }

        let song = serde_json::to_string_pretty(&item)
            .map_err(FrostElfError::FailedToSerialize)?
            .into_bytes();

        // IO impurity
        std::fs::write(file_path, song).map_err(FrostElfError::FailedToWriteIntoFile)
    }
}

impl TryFrom<SeldaTheListener> for ParastooTheKeeper {
    type Error = FrostElfError;

    fn try_from(selda: SeldaTheListener) -> Result<Self, Self::Error> {
        Ok(Self {
            // IO impurity
            vines: read_dir(&selda.0) // create a directory reader
                //then mapping the error of directory reading
                .map_err(FrostElfError::CouldNotReadTheDirectory)?
                // filtering all the items that are not ok
                .filter_map(Result::ok) // filter those that are ok
                // we only use json files
                .filter(|direntry| direntry.path().extension().map_or(false, |e| e == "json"))
                .map(|i| (i.path(), OnceCell::new()))
                .collect(),
            the_path: selda,
        })
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

    /// File exists.
    #[error("redmaple file already exists {0}")]
    FileExists(PathBuf),

    /// Multiple given file already exists.
    #[error("redmaple file does not exists {0}")]
    FileDoesNotExists(PathBuf),

    /// The file content could not be read.
    #[error("failed to read the content of the file: {0}")]
    FileReadFailed(std::io::Error),

    /// if a path is not given
    #[error("the given path does not exist {0}")]
    GivenPathDoesNotExist(PathBuf),

    /// indicates that the file at the given address does not exist.
    /// this should not happen.
    #[error("could not read the directory")]
    CouldNotReadTheDirectory(std::io::Error),
}
