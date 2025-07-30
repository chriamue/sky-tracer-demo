use sky_tracer::protocol::airports::AirportResponse;
use sky_tracer::protocol::flights::FlightResponse;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::routes::v1::airports::list_airports,
        crate::routes::v1::airports::get_airport,
        crate::routes::v1::flights::list_flights,
        crate::routes::v1::flights::post_flight,
        crate::routes::v1::flights::get_flight,
    ),
    components(
        schemas(
            AirportResponse,
            FlightResponse,
        )
    ),
    tags(
        (name = "Airports", description = "Airport lookup endpoints"),
        (name = "Flights", description = "Flight management endpoints"),
    )
)]
pub struct ApiDoc;
