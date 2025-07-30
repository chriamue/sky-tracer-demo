pub mod v1;

use axum::Router;

pub fn create_router() -> Router {
    Router::new().nest("/api/v1", v1::create_router())
}
