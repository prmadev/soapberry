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
    callers::{
        health_call::{
            error_handlr, marco_polo_response, marco_polo_response_handler,
            ConnectedHealthCheckClient, Error,
        },
        journey::{self, ConnectedJourneyClient},
    },
    client_configuration::{Commands, Config, ConfigurationError},
    grpc_definitions::{Body, Title},
};

#[tokio::main]
async fn main() -> Result<(), CommandLineError> {
    let conf = Config::try_from(std::env::args_os())?;
    router(conf.command(), conf.server_address()).await?;

    Ok(())
}

#[derive(thiserror::Error, Debug)]
enum CommandLineError {
    #[error("function could not connect to server: {0}")]
    Connection(#[from] tonic::transport::Error),

    #[error("failure during checking health: {0}")]
    CheckingHealth(#[from] Error),

    #[error("failure during working with journey: {0}")]
    JourneyDidWork(#[from] journey::Error),

    #[error("failure building configuration: {0}")]
    BuildingConfiguration(#[from] ConfigurationError),
}

async fn router(command: &Commands, server_address: SocketAddr) -> Result<(), CommandLineError> {
    match command {
        Commands::HealthCheck => {
            // making client
            let (mut client, request) = health_check(server_address).await?;

            //
            marco_polo_response_handler(String::from("Polo"))(
                marco_polo_response(client.inner_mut(), request, error_handlr).await?,
            )?;
        }
        Commands::New => {
            let mut client = ConnectedJourneyClient::connected_client(server_address)
                .await
                .map_err(CommandLineError::JourneyDidWork)?;

            let request = tonic::Request::new(kyushu::grpc_definitions::CreateEntryRequest {
                entry_title: Some(Title {
                    content: "some title".to_owned(),
                }),
                body: Some(Body {
                    content: ("some body".to_owned()),
                }),
            });

            let resp = client
                .inner_mut()
                .create_entry(request)
                .await
                .map_err(|e| println!("{e}"));
            println!("{resp:#?}");
        }
    };
    Ok(())
}

async fn health_check(
    server_address: SocketAddr,
) -> Result<
    (
        ConnectedHealthCheckClient,
        tonic::Request<kyushu::grpc_definitions::MarcoPoloRequest>,
    ),
    CommandLineError,
> {
    let client = ConnectedHealthCheckClient::connected_client(server_address)
        .await
        .map_err(CommandLineError::CheckingHealth)?;
    let request = tonic::Request::new(kyushu::grpc_definitions::MarcoPoloRequest {
        marco: Some(kyushu::grpc_definitions::Marco {
            content: String::from("Marco"),
        }),
    });
    Ok((client, request))
}
