use crate::routes::api;
use axum::{Router, response::Redirect, routing::get};
use sky_tracer::model::SatelliteStatus;
use sky_tracer::protocol::satellite::{
    CalculatePositionRequest, CalculatePositionResponse, CreateSatelliteRequest, SatelliteResponse,
    UpdateSatelliteStatusRequest,
};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(
        api::create_satellite,
        api::update_satellite_status,
        api::list_satellites,
        api::calculate_position
    ),
    components(
        schemas(
            CreateSatelliteRequest,
            UpdateSatelliteStatusRequest,
            SatelliteResponse,
            CalculatePositionRequest,
            CalculatePositionResponse,
            SatelliteStatus
        )
    ),
    tags(
        (name = "satellites", description = "Satellite management API")
    ),
    servers(
        (url = "/", description = "Local development server"),
        (url = "/api/v1/satellites", description = "Satellites API"),
        (url = "/api/v1/satellites/position", description = "Position calculation API")
    )
)]
pub struct ApiDoc;

pub fn routes<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    // Use SERVICE_NAME which contains the service name like "satellites"
    let service_name = std::env::var("SERVICE_NAME").unwrap_or_else(|_| "satellites".to_string());

    // Clone for the closure
    let redirect_path = format!("/{}/docs", service_name);
    let docs_path = format!("/{}/docs", service_name);
    let openapi_path = format!("/{}/docs/openapi.json", service_name);

    Router::new()
        // Redirect /api/docs to /{service_name}/docs (e.g., /satellites/docs)
        .route(
            "/api/docs/",
            get(move || {
                let redirect_path = redirect_path.clone();
                async move { Redirect::permanent(&redirect_path) }
            }),
        )
        // Serve docs at the full external path (e.g., /satellites/docs)
        .merge(SwaggerUi::new(docs_path).url(openapi_path, ApiDoc::openapi()))
}
