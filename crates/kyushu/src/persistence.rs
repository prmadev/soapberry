//! persistence contains a set of abstractions and connections for persistence of data

#[cfg(feature = "sqlite_store")]
pub mod sqlite_store;

pub mod structsy_store;
