use hyper::{Body, Request, Response};
use std::convert::Infallible;
use tracing::info;

pub async fn health_check(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let response_body = "OK";
    info!("Health check status: {}", response_body);
    Ok(Response::new(Body::from(response_body)))
}
