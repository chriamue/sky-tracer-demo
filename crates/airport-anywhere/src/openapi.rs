use crate::service;
use sky_tracer::protocol::airports::{
    AirportResponse, Position, SearchAirportsRequest, SearchAirportsResponse,
};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        service::list_airports,
        service::search_airports
    ),
    components(
        schemas(AirportResponse, Position, SearchAirportsRequest, SearchAirportsResponse)
    ),
    tags(
        (name = "airports", description = "Airport management API")
    ),
    servers(
        (url = "/", description = "Local server"),
        (url = "/airports", description = "Airports API server")
    )
)]
pub struct ApiDoc;
