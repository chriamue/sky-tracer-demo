use crate::routes;
use axum::Router;
use sky_tracer::protocol::NEXUS_API_PATH;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub fn app() -> Router {
    Router::new()
        .merge(SwaggerUi::new("/api/docs").url(
            format!("{}/api-docs/openapi.json", NEXUS_API_PATH),
            crate::openapi::ApiDoc::openapi(),
        ))
        .merge(routes::create_router())
}
