//! configuration of server binary

use std::{
    net::{AddrParseError, IpAddr, SocketAddr},
    num::ParseIntError,
    path::PathBuf,
    str::FromStr,
};

use clap::{command, Parser};

use crate::persistence::structsy_store::{DBFile, DBFileError, KnownDBFile};

/// [`Config`] holds the configuarion and command logic that the user runs to run [`kyushu_client`]
///
/// * `server_address`: is socket address in the form of 127.0.0.1:10000
pub struct Config {
    server_address: SocketAddr,
    db_address: PathBuf,
}

/// arguemnts that starts with the app
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// current working directory
    #[arg(short, long)]
    address: Option<String>,

    #[arg(short, long)]
    port: Option<u16>,

    #[arg(short, long)]
    db: Option<String>,

    #[arg(short, long)]
    create_db: bool,
}

/// these are the errors that happen when trying to form a configuration for client
#[derive(thiserror::Error, Debug)]
pub enum ConfigurationError {
    /// error that happen when trying to parse an integer out of an string
    #[error("Could not parsee integer: {0}")]
    ProblemParsingInteger(#[from] ParseIntError),

    /// error that happen when trying to pars an socket address from an string
    #[error("Could not parse address: {0}")]
    ProblemParsingTheAddress(#[from] AddrParseError),

    /// mising address
    #[error("you should pass in the server's ip address")]
    MissingAddressArgument,

    /// mising port
    #[error("you should pass in the server's port number")]
    MissingPortArgument,

    /// mising database file
    #[error(
        "you should add a database file, if not existing add the path that it should be located"
    )]
    MissingDatabaseArgument,

    /// could not parse path
    #[error("could not parse the path to database: {0}")]
    CouldNotParseDBPathArgument(std::convert::Infallible),

    /// could not determine the existence of the db file
    #[error("could not confirm the state of the database file : {0}")]
    CouldNotDetermineTheExistenceOfDBFile(DBFileError),

    /// database file at the given path is not found. If you want me to create one, pass the appropiate arguemnt
    #[error("database file at path {0} does not exist. If you want me to create, pass the appropiate flag")]
    CouldNotFindDatabaseFile(PathBuf),
}

impl TryFrom<std::env::ArgsOs> for Config {
    type Error = ConfigurationError;

    /// [`try_from`] parses the runtime arguments and returns a Result<[`Config`], [`ConfigurationError`]> struct
    ///
    /// # Errors
    ///
    /// may return all kinds of errors. all wrapped in [`ConfigurationError`]. for more details see
    /// there.
    fn try_from(value: std::env::ArgsOs) -> Result<Self, Self::Error> {
        let args = Args::parse_from(value);
        let serve = args
            .address
            .ok_or(ConfigurationError::MissingAddressArgument)?;
        let port = args.port.ok_or(ConfigurationError::MissingPortArgument)?;

        let sockadd = SocketAddr::new(IpAddr::from_str(&serve)?, port);
        let db = DBFile::from(
            PathBuf::from_str(&args.db.ok_or(ConfigurationError::MissingDatabaseArgument)?)
                .map_err(ConfigurationError::CouldNotParseDBPathArgument)?,
        );
        let dbfile = KnownDBFile::try_from(db.clone())
            .map_err(ConfigurationError::CouldNotDetermineTheExistenceOfDBFile)?;

        let db_result = match (&dbfile, &args.create_db) {
            (KnownDBFile::Existing(ref d), true | false) => Ok(d.inner()),
            (KnownDBFile::NotExisting(ref d), true) => Ok(d.inner()),
            (KnownDBFile::NotExisting(_), false) => Err(
                ConfigurationError::CouldNotFindDatabaseFile(db.into_inner()),
            ),
        };

        Ok(Self {
            server_address: sockadd,
            db_address: db_result?.clone(),
        })
    }
}

impl Config {
    /// returns the address of the server
    #[must_use]
    pub const fn server_address(&self) -> SocketAddr {
        self.server_address
    }

    /// Returns the address to the db file
    #[must_use]
    pub const fn db_address(&self) -> &PathBuf {
        &self.db_address
    }
}
