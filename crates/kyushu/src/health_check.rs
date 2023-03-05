//! `health_check` provides service logic for the health check service

use tonic::{async_trait, Request, Response, Status};

use crate::api::{
    health_check_service_server::HealthCheckService, MarcoPoloRequest, MarcoPoloResponse, Polo,
};

#[derive(Debug, Default)]
/// [`HealthSevice`] is a service provider for checking the health of the server
pub struct HealthSevice;

#[async_trait]
impl HealthCheckService for HealthSevice {
    async fn marco_polo(
        &self,
        request: Request<MarcoPoloRequest>,
    ) -> Result<Response<MarcoPoloResponse>, Status> {
        println!("got a request");
        let req = request.into_inner();

        match req.marco {
            Some(x) if x.content == "marco" => {
                let reply = MarcoPoloResponse {
                    polo: Some(Polo {
                        content: String::from("x"),
                    }),
                };
                let response = Response::new(reply);

                Ok(response)
            }
            Some(y) => Err(Status::new(
                tonic::Code::InvalidArgument,
                format!("Expected a marco, got {} instead", y.content),
            )),
            None => Err(Status::new(
                tonic::Code::InvalidArgument,
                "Expected a content",
            )),
        }
    }
}
