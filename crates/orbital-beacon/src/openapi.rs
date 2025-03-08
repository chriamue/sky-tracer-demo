use crate::service;
use sky_tracer::model::SatelliteStatus;
use sky_tracer::protocol::satellite::{
    CalculatePositionRequest, CalculatePositionResponse, CreateSatelliteRequest, SatelliteResponse,
    UpdateSatelliteStatusRequest,
};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        service::create_satellite,
        service::update_satellite_status,
        service::list_satellites,
        service::calculate_position
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
        (url = "/", description = "Local development server")
    )
)]
pub struct ApiDoc;
