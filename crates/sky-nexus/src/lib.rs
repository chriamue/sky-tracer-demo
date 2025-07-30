pub mod openapi;
pub mod routes;
pub mod services;

use axum::Router;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub fn app() -> Router {
    Router::new()
        .merge(
            SwaggerUi::new("/swagger-ui")
                .url("/api-docs/openapi.json", crate::openapi::ApiDoc::openapi()),
        )
        .merge(routes::create_router())
}
