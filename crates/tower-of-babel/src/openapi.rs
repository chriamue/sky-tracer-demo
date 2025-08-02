use sky_tracer::protocol::flights::ErrorResponse;
use utoipa::OpenApi;

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
