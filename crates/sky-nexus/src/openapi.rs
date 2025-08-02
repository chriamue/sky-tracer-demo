use sky_tracer::protocol::{
    airports::AirportResponse,
    flights::{CreateFlightRequest, FlightResponse},
    satellite::{
        CalculatePositionRequest, CalculatePositionResponse, CreateSatelliteRequest,
        SatelliteResponse, UpdateSatelliteStatusRequest,
    },
};
use utoipa::OpenApi;

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
        )
    ),
    tags(
        (name = "Airports", description = "Airport lookup endpoints"),
        (name = "Flights", description = "Flight management endpoints"),
        (name = "Satellites", description = "Satellite endpoints"),
    ),
    servers(
        (url = "/", description = "Local development server"),
        (url = "/api/v1/nexus", description = "Sky Nexus API base path")
    )
)]
pub struct ApiDoc;
