pub mod mcp;
pub mod v1;

use crate::mcp::AirportTools;
use axum::Router;
use rmcp::transport::streamable_http_server::{
    StreamableHttpService, session::local::LocalSessionManager,
};

pub fn create_router() -> Router {
    // Create the MCP service
    let mcp_service = StreamableHttpService::new(
        || Ok(AirportTools::new()),
        LocalSessionManager::default().into(),
        Default::default(),
    );

    Router::new()
        .nest("/api/v1", v1::create_router())
        .nest_service("/mcp", mcp_service)
}
