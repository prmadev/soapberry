//! persistence implementation using sqlite
//
// use std::path::PathBuf;
//
// use sqlx::{sqlite::SqliteConnectOptions, Pool, Sqlite, SqlitePool};
//
// /// [`DBFile`] is holder for a file that is supposed to be existing
// pub struct DBFile<State = NotKnown>(PathBuf, std::marker::PhantomData<State>);
//
// /// this is a zero sized struct holding state of [`DBFile`]
// /// for when the **file exists**
// pub struct Existing;
//
// /// this is a zero sized struct holding state of [`DBFile`]
// /// for when the file does **not exist**
// pub struct NotExisting;
//
// /// this is a zero sized struct holding state of [`DBFile`]
// /// for when the file is **not know to exist or not**
// pub struct NotKnown;
//
// /// errors that may returns when trying to check for the existence of database file
// #[derive(thiserror::Error, Debug)]
// pub enum DBFileError {
//     /// this happens when we are trying to validate the state of the db file
//     #[error("Could not check the status of db file {0}")]
//     FailedFileStatusCheck(#[from] std::io::Error),
// }
// impl<State> DBFile<State> {
//     const fn inner(&self) -> &PathBuf {
//         &self.0
//     }
//
//     #[allow(clippy::missing_const_for_fn)] // clippy thinks it can be const, while it cannot
//     fn into_inner(self) -> PathBuf {
//         self.0
//     }
// }
//
// impl From<PathBuf> for DBFile {
//     fn from(file: PathBuf) -> Self {
//         Self(file, std::marker::PhantomData::<NotKnown>)
//     }
// }
// enum KnownDBFile {
//     Existing(DBFile<Existing>),
//     NotExisting(DBFile<NotExisting>),
// }
// impl TryFrom<DBFile<NotKnown>> for KnownDBFile {
//     type Error = DBFileError;
//
//     fn try_from(value: DBFile<NotKnown>) -> Result<Self, Self::Error> {
//         let file_path = value.into_inner();
//         if !file_path.try_exists()? {
//             return Ok(Self::NotExisting(DBFile(
//                 file_path,
//                 std::marker::PhantomData::<NotExisting>,
//             )));
//         };
//         Ok(Self::Existing(DBFile(
//             file_path,
//             std::marker::PhantomData::<Existing>,
//         )))
//     }
// }
//
// struct LazyStore(Pool<Sqlite>);
//
// impl From<DBFile<Existing>> for LazyStore {
//     fn from(value: DBFile<Existing>) -> Self {
//         Self(SqlitePool::connect_lazy_with(
//             SqliteConnectOptions::new().filename(value.inner()),
//         ))
//     }
// }
//
// impl From<DBFile<NotExisting>> for LazyStore {
//     fn from(value: DBFile<NotExisting>) -> Self {
//         Self(SqlitePool::connect_lazy_with(
//             SqliteConnectOptions::new()
//                 .create_if_missing(true) // the main difference from an existing
//                 .filename(value.inner()),
//         ))
//     }
// }
