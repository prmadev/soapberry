//! [`health_respond`] provides service logic for the health check service

use tonic::{async_trait, Request, Response, Status};
use tracing::instrument;

use crate::grpc_definitions::{
    health_check_service_server::HealthCheckService, MarcoPoloRequest, MarcoPoloResponse, Polo,
};

#[derive(Debug, Default)]
/// [`HealthSevice`] is a service provider for checking the health of the server
pub struct Sevice;

#[async_trait]
impl HealthCheckService for Sevice {
    #[instrument(level = "debug" , fields(request_id = %uuid::Uuid::new_v4()), skip(request))]
    async fn marco_polo(
        &self,
        request: Request<MarcoPoloRequest>,
    ) -> Result<Response<MarcoPoloResponse>, Status> {
        match request.into_inner().marco {
            Some(x) if x.content == "Marco" => {
                let reply = MarcoPoloResponse {
                    polo: Some(Polo {
                        content: String::from("Polo"),
                    }),
                };

                let response = Response::new(reply);

                Ok(response)
            }
            Some(y) => Err(Status::new(
                tonic::Code::InvalidArgument,
                format!("Expected a Marco, got {} instead", y.content),
            )),
            None => Err(Status::new(
                tonic::Code::InvalidArgument,
                "Expected a content",
            )),
        }
    }
}
