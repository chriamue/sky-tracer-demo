use crate::service;
use sky_tracer::protocol::flights::{ErrorResponse, FlightPositionResponse, FlightResponse};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        service::list_flights_by_airport,
        service::get_flight_position
    ),
    components(
        schemas(
            FlightResponse,
            FlightPositionResponse,
            ErrorResponse
        )
    ),
    tags(
        (name = "flights", description = "Flight lookup operations")
    ),
    servers(
        (url = "/", description = "Local development server")
    )
)]
pub struct ApiDoc;
