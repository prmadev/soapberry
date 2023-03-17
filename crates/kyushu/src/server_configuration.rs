//! configuration of server binary

use std::{
    net::{AddrParseError, IpAddr, SocketAddr},
    num::ParseIntError,
    str::FromStr,
};

use clap::{command, Parser};

/// [`Config`] holds the configuarion and command logic that the user runs to run [`kyushu_client`]
///
/// * `server_address`: is socket address in the form of 127.0.0.1:10000
pub struct Config {
    server_address: SocketAddr,
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

        Ok(Self {
            server_address: sockadd,
        })
    }
}

impl Config {
    /// returns the address of the server
    #[must_use]
    pub const fn server_address(&self) -> SocketAddr {
        self.server_address
    }
}
