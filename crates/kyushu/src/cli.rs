//! cli holds information about the cli interface
pub mod printers;
pub use printers::*;
use redmaple::id::ID;
use std::{env::ArgsOs, path::PathBuf};

use clap::Parser;
use whirlybird::journey::{Body, Maple};

use crate::domain::requests::{Change, Information, Request};

//
// # type declaration
//

/// interpretations of valid arguments given to the program
#[derive(clap::Parser, Debug, Clone, PartialEq, Eq)]
#[command(author, version, about)]
pub struct Args {
    /// commands that run with this program
    #[command(subcommand)]
    pub command: Commands,

    /// the custom repo for the files
    #[arg(short, long)]
    pub file_store: Option<PathBuf>,
}

//
// # implementations
//

impl TryFrom<ArgsOs> for Args {
    type Error = ArgFromArgOSError;

    fn try_from(value: ArgsOs) -> Result<Self, Self::Error> {
        Self::try_parse_from(value).map_err(ArgFromArgOSError::CouldNotParseError)
    }
}

/// Errors for the conversion from [`ArgOs`] to [`Args`]
#[derive(thiserror::Error, Debug)]
pub enum ArgFromArgOSError {
    /// Couldnot parse!
    #[error("Could not parse {0}")]
    CouldNotParseError(#[from] clap::Error),
}

impl Args {
    /// Conversion to domain [`Request`]
    ///
    /// # Errors
    ///
    /// if there are inconsistencies and domain problems, this conversion will return an error
    #[allow(clippy::cast_sign_loss)] // timestamp is given in i64, but it can only be positive
    pub fn to_request(self) -> Result<crate::domain::requests::Request, ArgToDomainRequestError> {
        match self.command {
            Commands::Maple(maple_command) => match maple_command {
                MapleCommands::New { content } => {
                    let new_maple = Maple::new(
                        ID::new(
                            time::OffsetDateTime::now_local()
                                .map_err(ArgToDomainRequestError::TimeOffsetCouldNotBeGet)?
                                .unix_timestamp() as u64,
                        ),
                        Body::try_from(content)?,
                    );
                    let ch = Change::CreateNewMaple(new_maple);

                    Ok(Request::Change(ch))
                }

                MapleCommands::La => Ok(Request::Information(Information::ListEntries)),
            },
        }
    }
}

/// errors that happen when converting [`Args`] to [`Requst`] of domain
#[derive(thiserror::Error, Debug, Clone)]
pub enum ArgToDomainRequestError {
    /// Body Could not be built!
    #[error("body could not be built {0}")]
    BodyBuildingFailed(#[from] whirlybird::journey::BuildingError),

    /// Could not get the time!
    #[error("failed to get the time {0}")]
    TimeOffsetCouldNotBeGet(time::error::IndeterminateOffset),
}

//
// # type declaration
//

/// Commands that can be given to the program as argument
#[derive(clap::Subcommand, Debug, Clone, PartialEq, Eq)]
pub enum Commands {
    /// operations that happen to the [`Maple`]
    /// a [`Maple`] is the structure that holds a [`Body`]
    /// similar to an entry.
    #[command(subcommand)]
    Maple(MapleCommands),
}

//
// # type declaration
//

/// commands that come after [`maple`]
#[derive(clap::Subcommand, Debug, Clone, PartialEq, Eq)]
pub enum MapleCommands {
    /// creates a new [`maple`]
    New {
        /// content of the body of the [`maple`] [`Body`]
        content: String,
    },

    /// lists all the maples
    La,
}
