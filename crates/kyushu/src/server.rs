//! `kyushu` is an opinionated way of journaling and contemplating about your life.
//! this is a server binary made to act as daemon for the client
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

use kyushu::api::health_check_service_server::HealthCheckServiceServer;
use kyushu::responders::health_respond;
use kyushu::server_configuration::Config;
use kyushu::telemetry;
use tonic::transport::Server;
use tracing::instrument;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //
    // instrumentalizing
    //

    let subscriber = telemetry::subscriber(
        "kyushu_server".to_owned(),
        "info".to_owned(),
        std::io::stdout,
    );

    telemetry::init_subscriber(subscriber)?;

    //
    // configuration
    //

    let conf = Config::try_from(std::env::args_os())?;

    //
    // servers
    //

    // main
    let main_server_handler = MainServer::new(conf.server_address()).serve();
    main_server_handler.await?;

    Ok(())
}

#[derive(Debug)]
struct MainServer {
    address: SocketAddr,
    server: tonic::transport::server::Router,
}

impl MainServer {
    fn new(address: SocketAddr) -> Self {
        let health_service = health_respond::HealthSevice::default();

        let server = Server::builder().add_service(HealthCheckServiceServer::new(health_service));

        Self { address, server }
    }

    #[instrument(name = "Main server")]
    async fn serve(self) -> Result<(), tonic::transport::Error> {
        self.server.serve(self.address).await?;
        Ok(())
    }
}
