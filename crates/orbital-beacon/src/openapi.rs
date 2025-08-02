use crate::routes::api;
use sky_tracer::model::SatelliteStatus;
use sky_tracer::protocol::satellite::{
    CalculatePositionRequest, CalculatePositionResponse, CreateSatelliteRequest, SatelliteResponse,
    UpdateSatelliteStatusRequest,
};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        api::create_satellite,
        api::update_satellite_status,
        api::list_satellites,
        api::calculate_position
    ),
    components(
        schemas(
            CreateSatelliteRequest,
            UpdateSatelliteStatusRequest,
            SatelliteResponse,
            CalculatePositionRequest,
            CalculatePositionResponse,
            SatelliteStatus
        )
    ),
    tags(
        (name = "satellites", description = "Satellite management API")
    ),
    servers(
        (url = "/", description = "Local development server"),
        (url = "/api/v1/satellites", description = "Satellites API"),
        (url = "/api/v1/satellites/position", description = "Position calculation API")
    )
)]
pub struct ApiDoc;
