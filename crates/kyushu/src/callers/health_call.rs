//! [`health_call`] module returns the client logic for making request to server in order to check
//! if the server is working correctly

use std::net::SocketAddr;

use tonic::{codegen::http::uri::InvalidUri, transport::Channel};
use tracing::{error, instrument};

use crate::api::{
    health_check_service_client::HealthCheckServiceClient, Marco, MarcoPoloRequest, Polo,
};

/// [`HealthCheckClient`] holds a connection to the [`HealthCheckService`]
///
/// * `client`: is the underlying connection
#[derive(Debug)]
#[repr(transparent)] // because for now we are only using one object and making a wrapper around it
pub struct HealthCheckClient {
    client: HealthCheckServiceClient<Channel>,
}
const EXPECT: &str = "Polo";

impl HealthCheckClient {
    /// is a builder for the [`HealthCheckClient`]
    ///
    /// * `address`: is a the address of the server
    ///
    /// # Errors
    ///
    /// * [`HealthCheckError::ProblemConnecting`]: happens when client cannot connect
    pub async fn build(address: SocketAddr) -> Result<Self, HealthCheckError> {
        let s = format!("http://{address}");
        let client = HealthCheckServiceClient::connect(s)
            .await
            .map_err(|x| HealthCheckError::ProblemConnecting(Box::new(x)))?;

        Ok(Self { client })
    }

    /// checks the server, and essentailly pings it.
    /// this will only test a unary grpc connection.
    ///
    /// # Errors
    ///
    /// * [`HealthCheckError::OkStatus`]: may happen, though highly unlikely
    ///
    /// * [`HealthCheckError::ServerError`]: contains all other types of errors
    #[instrument]
    pub async fn marco_polo_test(&mut self) -> Result<(), HealthCheckError> {
        let request = tonic::Request::new(MarcoPoloRequest {
            marco: Some(Marco {
                content: "Marco".to_owned(),
            }),
        });

        let response = self
            .client
            .marco_polo(request)
            .await
            .map_err(|e| match e.code() {
                tonic::Code::Ok => HealthCheckError::OkStatus(e.message().to_owned()),
                x => HealthCheckError::ServerError(x),
            })?;

        check_if_polo(&response.into_inner().polo)
    }
}

/// [`HealthCheckError`] checks for any error that can be resulted when building and testing
/// the server during `HealthCheck`
#[derive(thiserror::Error, Debug)]
pub enum HealthCheckError {
    /// [`InvalidURI`] marks the problem with a URL socket that is not in a valid form
    #[error("Invalid URI: {0}")]
    InvalidURI(InvalidUri),

    /// [`ProblemConnecting`] error returns when a problem with connection with server accures
    #[error("could not connect: {0}")]
    ProblemConnecting(Box<dyn std::error::Error>),

    /// [`OkStatus`] error is a weird kind of error. this probably happens because the server has
    /// messed up and returned an `Ok` status when it should not have
    #[error("For some reason I've got an OK Status but in an error message : {0}")]
    OkStatus(String),

    /// [`ServerError`] happens when server returns an error
    #[error("could not connect {0}")]
    ServerError(tonic::Code),

    /// [`MissMatchResponse`] happens when server's response content does not match what we expect
    /// of it.
    #[error("got the wrong message: {0} I expected: {1} ")]
    MissMatchResponse(String, String),
}

fn check_if_polo(o: &Option<Polo>) -> Result<(), HealthCheckError> {
    match o {
        Some(p) if p.content == EXPECT => Ok(()),
        Some(p) => Err(HealthCheckError::MissMatchResponse(
            p.content.clone(),
            EXPECT.to_owned(),
        )),
        None => Err(HealthCheckError::MissMatchResponse(
            String::new(),
            EXPECT.to_owned(),
        )),
    }
}
