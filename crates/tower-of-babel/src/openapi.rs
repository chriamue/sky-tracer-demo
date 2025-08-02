use axum::{response::Redirect, routing::get, Router};
use sky_tracer::protocol::flights::ErrorResponse;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::routes::api::get_flights_by_airport,
        crate::routes::api::get_flight_position
    ),
    components(
        schemas(
            sky_tracer::protocol::flights::FlightResponse,
            sky_tracer::protocol::flights::FlightPositionResponse,
            ErrorResponse
        )
    ),
    tags(
        (name = "flights", description = "Flight lookup and aggregation operations")
    ),
    servers(
        (url = "/", description = "Local development server"),
        (url = "/api/v1/babel", description = "Babel API base path")
    )
)]
pub struct ApiDoc;

pub fn routes<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    // Use SERVICE_NAME which contains the service name like "babel"
    let service_name = std::env::var("SERVICE_NAME").unwrap_or_else(|_| "babel".to_string());

    // Clone for the closure
    let redirect_path = format!("/{}/docs", service_name);
    let docs_path = format!("/{}/docs", service_name);
    let openapi_path = format!("/{}/docs/openapi.json", service_name);

    Router::new()
        // Redirect /api/docs to /{service_name}/docs (e.g., /babel/docs)
        .route(
            "/api/docs/",
            get(move || {
                let redirect_path = redirect_path.clone();
                async move { Redirect::permanent(&redirect_path) }
            }),
        )
        // Serve docs at the full external path (e.g., /babel/docs)
        .merge(SwaggerUi::new(docs_path).url(openapi_path, ApiDoc::openapi()))
}
