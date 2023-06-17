//! cli holds information about the cli interface
pub mod printers;
pub use printers::*;
use std::env::ArgsOs;

use clap::Parser;
use whirlybird::journey::Body;

use crate::domain::requests::{Change, Information, Request};

//
// # type declaration
//

#[derive(clap::Parser, Debug, Clone, PartialEq, Eq)]
#[command(author, version, about)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,

    #[arg(short, long)]
    pub file_store: Option<std::path::PathBuf>,
}

//
// # implementations
//

impl TryFrom<ArgsOs> for Args {
    type Error = ArgFromArgOSError;

    fn try_from(value: ArgsOs) -> Result<Self, Self::Error> {
        Args::try_parse_from(value).map_err(ArgFromArgOSError::CouldNotParseError)
    }
}
#[derive(thiserror::Error, Debug)]
pub enum ArgFromArgOSError {
    /// Couldnot parse!
    #[error("Could not parse {0}")]
    CouldNotParseError(#[from] clap::Error),
}

impl TryInto<crate::domain::requests::Request> for Args {
    fn try_into(self) -> Result<crate::domain::requests::Request, Self::Error> {
        match self.command {
            Commands::Maple(maple_command) => match maple_command {
                MapleCommands::New { content } => {
                    let new_maple = whirlybird::journey::Maple::new(
                        redmaple::id::ID::new(uuid::Uuid::new_v4()),
                        Body::try_from(content)?,
                    );
                    let ch = Change::CreateNewMaple(new_maple);
                    Ok(Request::Change(ch))
                }
                MapleCommands::ListAll => Ok(Request::Information(Information::ListEntries)),
            },
        }
    }

    type Error = ArgToDomainRequestError;
}

#[derive(thiserror::Error, Debug, Clone)]
pub enum ArgToDomainRequestError {
    /// Body Could not be built!
    #[error("body could not be built {0}")]
    BodyBuildingFailed(#[from] whirlybird::journey::BuildingError),
}

//
// # type declaration
//

#[derive(clap::Subcommand, Debug, Clone, PartialEq, Eq)]
pub enum Commands {
    #[command(subcommand)]
    Maple(MapleCommands),
}

//
// # type declaration
//

#[derive(clap::Subcommand, Debug, Clone, PartialEq, Eq)]
pub enum MapleCommands {
    // creates a new entry
    New { content: String },
    // lists all the entries
    ListAll,
}
