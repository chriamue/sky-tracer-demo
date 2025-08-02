use crate::routes;
use axum::{response::Redirect, routing::get, Router};
use sky_tracer::protocol::flights;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(
        routes::create_flight,
        routes::list_flights,
        routes::get_flight_position,
    ),
    components(
        schemas(
            flights::CreateFlightRequest,
            flights::FlightResponse,
            flights::FlightPositionResponse,
            flights::ListFlightsRequest
        )
    ),
    tags(
        (name = "flights", description = "Flight management API")
    ),
    servers(
        (url = "/", description = "Local development server"),
        (url = "/flights", description = "Flight API v1"),
    )
)]
pub struct ApiDoc;

pub fn routes<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    // Use SERVICE_NAME which contains the service name like "flights"
    let service_name = std::env::var("SERVICE_NAME").unwrap_or_else(|_| "flights".to_string());

    // Clone for the closure
    let redirect_path = format!("/{}/docs", service_name);
    let docs_path = format!("/{}/docs", service_name);
    let openapi_path = format!("/{}/docs/openapi.json", service_name);

    Router::new()
        // Redirect /api/docs to /{service_name}/docs (e.g., /flights/docs)
        .route(
            "/api/docs/",
            get(move || {
                let redirect_path = redirect_path.clone();
                async move { Redirect::permanent(&redirect_path) }
            }),
        )
        // Serve docs at the full external path (e.g., /flights/docs)
        .merge(SwaggerUi::new(docs_path).url(openapi_path, ApiDoc::openapi()))
}
