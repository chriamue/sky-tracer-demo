use crate::routes;
use sky_tracer::protocol::flights;
use utoipa::OpenApi;

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
