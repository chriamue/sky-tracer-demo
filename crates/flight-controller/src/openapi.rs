use crate::routes;
use sky_tracer::protocol::flights;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        routes::create_flight,
        routes::list_flights,
    ),
    components(
        schemas(
            flights::CreateFlightRequest,
            flights::FlightResponse,
            flights::ListFlightsRequest
        )
    ),
    tags(
        (name = "flights", description = "Flight management API")
    ),
    servers(
        (url = "/flights/", description = "Flight API"),
        (url = "/", description = "Local development server")
    )
)]
pub struct ApiDoc;
