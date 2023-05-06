//! the main logic callers of the program

use std::net::SocketAddr;

use tonic::transport::Channel;

use crate::grpc_definitions::journey_service_client::JourneyServiceClient;

/// Holds a guranteed [`JourneyServiceClient`]
#[derive(Debug)]
pub struct ConnectedJourneyClient(JourneyServiceClient<Channel>);

impl ConnectedJourneyClient {
    /// is a builder for the [`HealthCheckClient`]
    ///
    /// * `address`: is a the address of the server
    ///
    /// # Errors
    ///
    /// * [`HealthCheckError::ProblemConnecting`]: happens when client cannot connect
    pub async fn connected_client(address: SocketAddr) -> Result<Self, Error> {
        let client = JourneyServiceClient::connect(format!("http://{address}"))
            .await
            .map_err(|x| Error::ProblemConnecting(Box::new(x)))?;

        Ok(Self(client))
    }

    /// Exposes the inner client for this type
    pub fn inner_mut(&mut self) -> &mut JourneyServiceClient<Channel> {
        &mut self.0
    }
}

/// [`HealthCheckError`] checks for any error that can be resulted when building and testing
/// the server during `HealthCheck`
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// [`ProblemConnecting`] error returns when a problem with connection with server accures
    #[error("could not connect: {0}")]
    ProblemConnecting(Box<dyn std::error::Error>),
}
