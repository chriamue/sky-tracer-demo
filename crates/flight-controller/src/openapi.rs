use crate::service;
use sky_tracer::protocol::flights;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        service::create_flight,
        service::list_flights,
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
