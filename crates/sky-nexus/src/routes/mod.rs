pub mod v1;

use crate::mcp::{AirportTools, SatelliteTools};
use axum::Router;
use rmcp::transport::streamable_http_server::{
    StreamableHttpService, session::local::LocalSessionManager,
};

pub fn create_router() -> Router {
    let airports_mcp_service = StreamableHttpService::new(
        || Ok(AirportTools::new()),
        LocalSessionManager::default().into(),
        Default::default(),
    );

    let satellites_mcp_service = StreamableHttpService::new(
        || Ok(SatelliteTools::new()),
        LocalSessionManager::default().into(),
        Default::default(),
    );

    Router::new()
        .nest("/api/v1", v1::create_router())
        .nest_service("/mcp/airports", airports_mcp_service)
        .nest_service("/mcp/satellites", satellites_mcp_service)
}
