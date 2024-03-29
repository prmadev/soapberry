//! parsing of arguments

use redmaple::id::ID;
use std::{env::ArgsOs, path::PathBuf};
use time::OffsetDateTime;

use clap::{arg, Parser};
use whirlybird::journey::{Body, Maple};

use crate::domain::requests::{Change, Information, Request};

/// interpretations of valid arguments given to the program
#[derive(clap::Parser, Debug, Clone, PartialEq, Eq)]
#[command(author)]
#[command(about = "Local-First, Event-Driven Journaling")]
#[command(version)]
#[command(bin_name = "kyushu")]
#[command(name = "kyushu")]
pub struct Args {
    /// commands that run with this program
    #[command(subcommand)]
    pub command: Commands,

    /// the custom repo for the files
    #[arg(short, long)]
    pub file_store: Option<PathBuf>,
}

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
    pub fn to_request(
        self,
        time_now: OffsetDateTime,
    ) -> Result<crate::domain::requests::Request, ArgToDomainRequestError> {
        match self.command {
            Commands::Maple(maple_command) => match maple_command {
                // request the creation of a brand new [`Maple`]
                MapleCommands::New {
                    content,
                    time_given,
                } => {
                    let new_maple = Maple::new(
                        ID::from(time_given.unwrap_or(time_now.unix_timestamp_nanos())),
                        Body::try_from(content)?,
                    );

                    let ch = Change::CreateNewMaple(new_maple);

                    let time_of_change = time_given
                        .map(OffsetDateTime::from_unix_timestamp_nanos)
                        .transpose()
                        .map_err(ArgToDomainRequestError::FailedToParseTime)?
                        .unwrap_or(time_now);

                    Ok(Request::Change((time_of_change, ch)))
                }

                // Request for listing of all maple
                MapleCommands::List => Ok(Request::Information(Information::ListEntries)),

                // Request a updating of body
                MapleCommands::Water {
                    content,
                    maple_id,
                    time_given,
                } => {
                    let time_of_change = time_given
                        .map(OffsetDateTime::from_unix_timestamp_nanos)
                        .transpose()
                        .map_err(ArgToDomainRequestError::FailedToParseTime)?
                        .unwrap_or(time_now);

                    Ok(Request::Change((
                        time_of_change,
                        Change::UpdateMapleBody(ID::from(maple_id), Body::try_from(content)?),
                    )))
                }

                MapleCommands::Link {
                    maple_from,
                    maple_to,
                    why,
                    time_given,
                } => {
                    let time_of_change = time_given
                        .map(OffsetDateTime::from_unix_timestamp_nanos)
                        .transpose()
                        .map_err(ArgToDomainRequestError::FailedToParseTime)?
                        .unwrap_or(time_now);

                    Ok(Request::Change((
                        time_of_change,
                        Change::AddLinkToMaple {
                            from: maple_from.into(),
                            to: maple_to.into(),
                            why,
                        },
                    )))
                }
                MapleCommands::Dislink {
                    link_to_remove,
                    time_given,
                } => {
                    let time_of_change = time_given
                        .map(OffsetDateTime::from_unix_timestamp_nanos)
                        .transpose()
                        .map_err(ArgToDomainRequestError::FailedToParseTime)?
                        .unwrap_or(time_now);
                    Ok(Request::Change((
                        time_of_change,
                        Change::Dislink {
                            link_id: ID::from(link_to_remove),
                        },
                    )))
                }
            },
        }
    }
}

/// errors that happen when converting [`Args`] to [`Requst`] of domain
#[derive(thiserror::Error, Debug, Clone)]
pub enum ArgToDomainRequestError {
    /// Body Could not be built!
    #[error("body could not be built: {0}")]
    BodyBuildingFailed(#[from] whirlybird::journey::BuildingError),

    /// Could not get the time!
    #[error("failed to get the time: {0}")]
    TimeOffsetCouldNotBeGet(time::error::IndeterminateOffset),

    /// Could Not Parse Time
    #[error("failed to parse time: {0}")]
    FailedToParseTime(time::error::ComponentRange),
}

/// Program arguments for issuing commands.
#[derive(clap::Subcommand, Debug, Clone, PartialEq, Eq)]
pub enum Commands {
    /// Operations performed on a maple structure.
    /// A maple serves as the container for events,
    /// analogous to an entry, but expressed in the form of events rather than state.
    #[command(subcommand)]
    Maple(MapleCommands),
}

/// Commands related to maple operations.
#[derive(clap::Subcommand, Debug, Clone, PartialEq, Eq)]
pub enum MapleCommands {
    /// Creates a new maple.
    #[command()]
    New {
        #[arg(value_name = "CONTENT")]
        /// Content of the body of the maple's Body.
        content: String,

        /// Time of the creation of the new event. This is used for IDs as well.
        #[arg(value_name = "TIME")]
        time_given: Option<i128>,
    },

    /// Lists all the maples.
    List,

    /// Updates the Body of a maple.
    #[command()]
    Water {
        /// ID of the maple for which the body is being updated.
        #[arg(value_name = "MAPLEID")]
        maple_id: i128,

        #[arg(value_name = "BODY")]
        /// Content of the body of the maple's Body.
        content: String,

        /// Time of the updating of maple. This is used for IDs as well.
        #[arg(value_name = "TIME")]
        time_given: Option<i128>,
    },

    /// links a maple to another
    Link {
        /// ID of the maple which we are linking from.
        #[arg(value_name = "Maple ID Of Origin")]
        maple_from: i128,
        /// ID of the maple which we are linking to.
        #[arg(value_name = "Maple ID Of Target")]
        maple_to: i128,
        /// Why the link?
        #[arg(value_name = "Why are you linking?")]
        why: String,

        /// Time of the linking. This is used for IDs as well.
        #[arg(value_name = "TIME")]
        time_given: Option<i128>,
    },

    /// Dislink the id
    Dislink {
        /// ID of the Link which we are removing the link.
        #[arg(value_name = "Link ID to remove")]
        link_to_remove: i128,

        /// Time of the linking. This is used for IDs as well.
        #[arg(value_name = "TIME")]
        time_given: Option<i128>,
    },
}

/// Commands related to housing maples.
#[derive(clap::Subcommand, Debug, Clone, PartialEq, Eq)]
pub enum HouseCommands {
    /// Creates a new house.
    #[command()]
    New {},
}
