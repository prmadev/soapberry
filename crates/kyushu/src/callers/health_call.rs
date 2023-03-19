//! [`health_call`] module returns the client logic for making request to server in order to check
//! if the server is working correctly

use std::net::SocketAddr;

use tonic::{codegen::http::uri::InvalidUri, transport::Channel, Request, Response, Status};
use tracing::error;

use crate::api::{
    health_check_service_client::HealthCheckServiceClient, MarcoPoloRequest, MarcoPoloResponse,
};

/// [`HealthCheckClient`] holds a connection to the [`HealthCheckService`]
///
/// * `client`: is the underlying connection
#[derive(Debug)]
pub struct ConnectedHealthCheckClient(HealthCheckServiceClient<Channel>);

impl ConnectedHealthCheckClient {
    /// is a builder for the [`HealthCheckClient`]
    ///
    /// * `address`: is a the address of the server
    ///
    /// # Errors
    ///
    /// * [`HealthCheckError::ProblemConnecting`]: happens when client cannot connect
    pub async fn connected_client(address: SocketAddr) -> Result<Self, HealthCheckError> {
        let client = HealthCheckServiceClient::connect(format!("http://{address}"))
            .await
            .map_err(|x| HealthCheckError::ProblemConnecting(Box::new(x)))?;

        Ok(Self(client))
    }

    /// Exposes the inner client for this type
    pub fn inner_mut(&mut self) -> &mut HealthCheckServiceClient<Channel> {
        &mut self.0
    }
}

/// response getter
/// TODO make better documentation
///
/// # Errors
pub async fn marco_polo_response(
    client: &mut HealthCheckServiceClient<Channel>,
    request: Request<MarcoPoloRequest>,
    error_handlr: impl Fn(Status) -> HealthCheckError,
) -> Result<Response<MarcoPoloResponse>, HealthCheckError> {
    let resp = client.marco_polo(request).await.map_err(error_handlr)?;

    Ok(resp)
}

#[allow(clippy::needless_pass_by_value)] // this is in order to recieve the status and being acceptable by compiler for map_err
/// an error handler which checks the status for different errors and returns the appropiate error
pub fn error_handlr(status: Status) -> HealthCheckError {
    match status.code() {
        tonic::Code::Ok => HealthCheckError::OkStatus(status.message().to_owned()),
        x => HealthCheckError::ServerError(x),
    }
}

/// an example response handler for Marco Polo
///
/// # Errors
///
/// it may return errors to be used by the test
pub fn marco_polo_response_handler(
    expected_response_content: String,
) -> impl Fn(Response<MarcoPoloResponse>) -> Result<(), HealthCheckError> {
    move |response: Response<MarcoPoloResponse>| -> Result<(), HealthCheckError> {
        response
            .into_inner()
            .polo
            // matching the empty polo
            .ok_or(HealthCheckError::MissMatchResponse(
                String::new(),
                expected_response_content.clone(),
            ))
            // matching non polo response
            .map(|p| {
                if p.content == expected_response_content {
                    return Ok(());
                }
                Err(HealthCheckError::MissMatchResponse(
                    p.content,
                    expected_response_content.clone(),
                ))
            })?
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
