use crate::mcp::AirportTools;
use axum::{body::Body, extract::Request, response::Response};
use http_body_util::BodyExt;
use rmcp::transport::streamable_http_server::{
    StreamableHttpService, session::local::LocalSessionManager,
};
use std::convert::Infallible;
use tower::Service;

pub async fn mcp_handler(mut request: Request) -> Result<Response, Infallible> {
    let mut service = StreamableHttpService::new(
        || Ok(AirportTools::new()),
        LocalSessionManager::default().into(),
        Default::default(),
    );

    // Use the Service trait directly
    match Service::call(&mut service, request).await {
        Ok(response) => {
            // Convert the response body to the expected type
            let (parts, body) = response.into_parts();
            let body = Body::from_stream(body.into_data_stream());
            Ok(Response::from_parts(parts, body))
        }
        Err(_) => Ok(Response::builder()
            .status(500)
            .body(Body::from("Internal server error"))
            .unwrap()),
    }
}
