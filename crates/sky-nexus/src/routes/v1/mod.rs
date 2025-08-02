pub mod airports;
pub mod datetime;
pub mod flights;
pub mod satellites;

use axum::Router;

pub fn create_router() -> Router {
    Router::new()
        .nest("/nexus/airports", airports::router())
        .nest("/nexus/flights", flights::router())
        .nest("/nexus/satellites", satellites::router())
        .nest("/nexus/datetime", datetime::router())
}
