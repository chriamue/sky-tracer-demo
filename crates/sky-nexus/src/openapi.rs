use crate::models::datetime::*;
use axum::{Router, response::Redirect, routing::get};
use sky_tracer::protocol::{
    airports::AirportResponse,
    flights::{CreateFlightRequest, FlightResponse},
    satellite::{
        CalculatePositionRequest, CalculatePositionResponse, CreateSatelliteRequest,
        SatelliteResponse, UpdateSatelliteStatusRequest,
    },
};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::routes::v1::airports::list_airports,
        crate::routes::v1::airports::get_airport,
        crate::routes::v1::flights::list_flights,
        crate::routes::v1::flights::post_flight,
        crate::routes::v1::flights::get_flight,
        crate::routes::v1::satellites::list_satellites,
        crate::routes::v1::satellites::post_satellite,
        crate::routes::v1::satellites::put_satellite_status,
        crate::routes::v1::satellites::post_calculate_position,
        crate::routes::v1::datetime::get_current_datetime_route,
        crate::routes::v1::datetime::get_aviation_times_route,
        crate::routes::v1::datetime::compare_timezones_route,
    ),
    components(
        schemas(
            AirportResponse,
            FlightResponse,
            CreateFlightRequest,
            SatelliteResponse,
            CreateSatelliteRequest,
            UpdateSatelliteStatusRequest,
            CalculatePositionRequest,
            CalculatePositionResponse,
            DateTimeResponse,
            AviationTimesResponse,
            AviationTimeZone,
            GetDateTimeQuery,
            TimezoneComparisonRequest,
            TimezoneComparisonResponse,
            TimezoneInfo,
        )
    ),
    tags(
        (name = "Airports", description = "Airport lookup endpoints"),
        (name = "Flights", description = "Flight management endpoints"),
        (name = "Satellites", description = "Satellite endpoints"),
        (name = "DateTime", description = "Date and time utilities for aviation operations"),
    ),
    servers(
        (url = "/", description = "Local development server"),
        (url = "/api/v1/nexus", description = "Sky Nexus API base path")
    )
)]
pub struct ApiDoc;

pub fn routes<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    // Use SERVICE_NAME which contains the service name like "nexus"
    let service_name = std::env::var("SERVICE_NAME").unwrap_or_else(|_| "nexus".to_string());

    // Clone for the closure
    let redirect_path = format!("/{}/docs", service_name);
    let docs_path = format!("/{}/docs", service_name);
    let openapi_path = format!("/{}/docs/openapi.json", service_name);

    Router::new()
        // Redirect /api/docs to /{service_name}/docs (e.g., /nexus/docs)
        .route(
            "/api/docs/",
            get(move || {
                let redirect_path = redirect_path.clone();
                async move { Redirect::permanent(&redirect_path) }
            }),
        )
        // Serve docs at the full external path (e.g., /nexus/docs)
        .merge(SwaggerUi::new(docs_path).url(openapi_path, ApiDoc::openapi()))
}
