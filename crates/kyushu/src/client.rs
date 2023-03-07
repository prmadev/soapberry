//! `kyushu` is an opinionated way of journaling and contemplating about your life.
//!
//! this is a client library made to act as way to communicate with the daemon and be used by the
//! users
//!
//! `kyushu` is proudly standing on the shoulders of [`RedMaple`](https://crates.io/crates/redmaple) library.
#![deny(missing_docs)]
#![deny(clippy::expect_used)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::indexing_slicing)]
#![deny(clippy::panic)]
#![warn(
    rust_2018_idioms,
    clippy::pedantic,
    clippy::cargo,
    clippy::clone_on_ref_ptr,
    clippy::default_numeric_fallback,
    clippy::string_to_string,
    clippy::unnecessary_self_imports,
    clippy::str_to_string,
    clippy::same_name_method,
    clippy::rc_buffer,
    clippy::panic_in_result_fn,
    clippy::multiple_inherent_impl,
    clippy::map_err_ignore,
    clippy::if_then_some_else_none,
    clippy::empty_structs_with_brackets,
    clippy::useless_let_if_seq,
    clippy::use_self,
    clippy::missing_const_for_fn,
    clippy::cognitive_complexity,
    clippy::self_named_constructors
)]
#![allow(clippy::multiple_crate_versions)]
#![cfg_attr(docsrs, feature(doc_cfg))]

use std::net::SocketAddr;

use kyushu::{
    callers::health_call::{HealthCheckClient, HealthCheckError},
    client_configuration::{Commands, Config, ConfigurationError},
};

#[tokio::main]
async fn main() -> Result<(), CommandLineError> {
    let conf = Config::build()?;
    route_command(conf.command(), conf.server_address()).await?;
    Ok(())
}

#[derive(thiserror::Error, Debug)]
enum CommandLineError {
    #[error("function could not connect to server: {0}")]
    Connection(#[from] tonic::transport::Error),

    #[error("failure during checking health: {0}")]
    CheckingHealth(#[from] HealthCheckError),

    #[error("failure building configuration: {0}")]
    BuildingConfiguration(#[from] ConfigurationError),
}

async fn route_command(
    command: &Commands,
    server_address: SocketAddr,
) -> Result<(), CommandLineError> {
    match command {
        Commands::HealthCheck => {
            let mut client = HealthCheckClient::build(server_address)
                .await
                .map_err(CommandLineError::CheckingHealth)?;
            client.marco_polo_test().await?;
        }
    };
    Ok(())
}
