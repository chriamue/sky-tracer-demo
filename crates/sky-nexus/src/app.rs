use crate::{openapi, routes};
use axum::Router;

pub fn app() -> Router {
    Router::new()
        .merge(openapi::routes())
        .merge(routes::create_router())
}
