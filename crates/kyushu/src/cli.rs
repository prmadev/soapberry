//! cli holds information about the cli interface

use std::env::ArgsOs;

use clap::Parser;

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
