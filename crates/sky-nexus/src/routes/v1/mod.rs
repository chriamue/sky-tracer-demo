pub mod airports;
pub mod flights;

use axum::Router;

pub fn create_router() -> Router {
    Router::new()
        .nest("/airports", airports::router())
        .nest("/flights", flights::router())
}
