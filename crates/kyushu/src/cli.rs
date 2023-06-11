//! cli holds information about the cli interface

use std::{env::ArgsOs, time};

use clap::Parser;
use whirlybird::journey::Body;

use crate::domain::requests::{Change, Request};

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

impl From<ArgsOs> for Args {
    fn from(value: ArgsOs) -> Self {
        Args::parse_from(value)
    }
}

impl TryInto<crate::domain::requests::Request> for Args {
    fn try_into(self) -> Result<crate::domain::requests::Request, Self::Error> {
        match self.command {
            Commands::Entry(entry_command) => match entry_command {
                EntryCommands::New { content } => {
                    let new_entry = whirlybird::journey::Entry::new(
                        redmaple::id::ID::new(uuid::Uuid::new_v4()),
                        time::SystemTime::now(),
                        Some(Body::build(content)?),
                    );
                    let ch = Change::CreateNewEntry(new_entry);
                    Ok(Request::Change(ch))
                }
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
    Entry(EntryCommands),
}

//
// # type declaration
//

#[derive(clap::Subcommand, Debug, Clone, PartialEq, Eq)]
pub enum EntryCommands {
    // creates a new entry
    New { content: String },
}
