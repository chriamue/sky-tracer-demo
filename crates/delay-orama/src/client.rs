use reqwest::Client;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_tracing::TracingMiddleware;

pub fn create_client() -> ClientWithMiddleware {
    ClientBuilder::new(Client::new())
        .with(TracingMiddleware::default())
        .build()
}
