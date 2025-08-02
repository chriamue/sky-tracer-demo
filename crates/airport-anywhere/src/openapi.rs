use crate::routes;
use axum::{response::Redirect, routing::get, Router};
use sky_tracer::protocol::airports::{
    AirportResponse, Position, SearchAirportsRequest, SearchAirportsResponse,
};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(
        routes::list_airports,
        routes::search_airports
    ),
    components(
        schemas(AirportResponse, Position, SearchAirportsRequest, SearchAirportsResponse)
    ),
    tags(
        (name = "airports", description = "Airport management API")
    ),
    servers(
        (url = "/", description = "Local server"),
        (url = "/airports", description = "Airports API server")
    )
)]
pub struct ApiDoc;

pub fn routes<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    // Use SERVICE_NAME which contains the service name like "airports"
    let service_name = std::env::var("SERVICE_NAME").unwrap_or_else(|_| "airports".to_string());

    // Clone for the closure
    let redirect_path = format!("/{}/docs", service_name);
    let docs_path = format!("/{}/docs", service_name);
    let openapi_path = format!("/{}/docs/openapi.json", service_name);

    Router::new()
        // Redirect /api/docs to /{service_name}/docs (e.g., /airports/docs)
        .route(
            "/api/docs/",
            get(move || {
                let redirect_path = redirect_path.clone();
                async move { Redirect::permanent(&redirect_path) }
            }),
        )
        // Serve docs at the full external path (e.g., /airports/docs)
        .merge(SwaggerUi::new(docs_path).url(openapi_path, ApiDoc::openapi()))
}
