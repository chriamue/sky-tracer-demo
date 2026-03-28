pub mod v1;

use crate::mcp::SkyNexusTools;
use axum::Router;
use rmcp::transport::streamable_http_server::{
    StreamableHttpService, session::local::LocalSessionManager,
};

pub fn create_router() -> Router {
    let mcp_service = StreamableHttpService::new(
        || Ok(SkyNexusTools::new()),
        LocalSessionManager::default().into(),
        Default::default(),
    );

    Router::new()
        .nest("/api/v1", v1::create_router())
        .nest_service("/mcp", mcp_service)
}
