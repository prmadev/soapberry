//! configuration of server binary

use std::{
    net::{AddrParseError, IpAddr, SocketAddr},
    num::ParseIntError,
    str::FromStr,
};

use clap::{command, Parser};
use inquire::{validator::Validation, InquireError, Text};
use validators::prelude::validators_prelude;

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
    /// error that happen when trying to get answer interactively from user
    #[error("could not get response from you: {0}")]
    InquireError(#[from] InquireError),

    /// error that happen when trying to parse an integer out of an string
    #[error("Could not parsee integer: {0}")]
    ProblemParsingInteger(#[from] ParseIntError),

    /// error that happen when trying to pars an socket address from an string
    #[error("Could not parsee address: {0}")]
    ProblemParsingTheAddress(#[from] AddrParseError),
}

impl Config {
    /// [`build`] parses the runtime arguments and returns a [`Config`] struct
    ///
    /// # Errors
    ///
    /// may return all kinds of errors. all wrapped in [`ConfigurationError`]. for more details see
    /// there.
    pub fn build() -> Result<Self, ConfigurationError> {
        let args = Args::parse();

        Ok(Self {
            server_address: SocketAddr::new(
                IpAddr::from_str(&match &args.address {
                    Some(a) => a.clone(),
                    None => ask_for_address()?,
                })?,
                match args.port {
                    Some(p) => p,
                    None => ask_for_port(9000)?,
                },
            ),
        })
    }

    /// returns the address of the server
    #[must_use]
    pub const fn server_address(&self) -> SocketAddr {
        self.server_address
    }
}

/// this is a function that asks the user for an IP to the server interactively
fn ask_for_address() -> Result<String, ConfigurationError> {
    let address_validator = |input: &str| {
        if validators_prelude::Ipv4Addr::from_str(input).is_err() && input != "localhost" {
            Ok(Validation::Invalid(
                "the address does not seem to be a valid address".into(),
            ))
        } else {
            Ok(Validation::Valid)
        }
    };

    Ok(
        match Text::new("what is the server's address")
            .with_validator(address_validator)
            .with_default("127.0.0.1")
            .with_placeholder("127.0.0.1")
            .with_help_message(
                "the address of the server. can be ip address like 0.0.0.0 or it can be localhost.",
            )
            .prompt()?
            .as_str()
        {
            "localhost" => "127.0.0.1".to_owned(),
            x => x.to_owned(),
        },
    )
}

/// this is a function that asks the user for an port number of the server interactively
fn ask_for_port(default_port: u16) -> Result<u16, ConfigurationError> {
    let port_validator = |input: &str| match u16::from_str(input) {
        Ok(_) => Ok(Validation::Valid),
        Err(_) => Ok(Validation::Invalid(
            "the port does not seem to be a valid port number".into(),
        )),
    };

    Ok(u16::from_str(
        &Text::new("what is the server's port ")
            .with_validator(port_validator)
            .with_default(&format!("{default_port}"))
            .with_placeholder(&format!("{default_port}"))
            .with_help_message("the port number of the server, can be any number from 0 to 65536.")
            .prompt()?,
    )?)
}
