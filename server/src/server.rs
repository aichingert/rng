use tonic::{Request, Response, Status};

use crate::suptac::greeting_server::Greeting; 
use crate::suptac::{HelloRequest, HelloResponse};

#[derive(Default)]
pub struct Greeter {}

#[tonic::async_trait]
impl Greeting for Greeter {
    async fn greet(
        &self, 
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloResponse>, Status> {
        let r = request.into_inner();

        println!("Got a request for: {:?}", &r);

        let reply = HelloResponse {
            greeting: format!("hello, {}!", r.name),
        };

        Ok(Response::new(reply))
    }
}
