use crate::services::satellites::{
    calculate_position, create_satellite, fetch_satellites, update_satellite_status,
};
use axum::{
    Json, Router,
    extract::Path,
    routing::{get, post, put},
};
use sky_tracer::protocol::satellite::{
    CalculatePositionRequest, CalculatePositionResponse, CreateSatelliteRequest, SatelliteResponse,
    UpdateSatelliteStatusRequest,
};
use uuid::Uuid;

pub fn router() -> Router {
    Router::new()
        .route("/", get(list_satellites).post(post_satellite))
        .route("/{id}/status", put(put_satellite_status))
        .route("/position", post(post_calculate_position))
}

#[utoipa::path(
    get,
    path = "/api/v1/nexus/satellites",
    responses(
        (status = 200, description = "List all satellites", body = [SatelliteResponse])
    ),
    tag = "Satellites"
)]
pub async fn list_satellites() -> Json<Vec<SatelliteResponse>> {
    let satellites = fetch_satellites().await.unwrap_or_default();
    Json(satellites)
}

#[utoipa::path(
    post,
    path = "/api/v1/nexus/satellites",
    request_body = CreateSatelliteRequest,
    responses(
        (status = 201, description = "Satellite created", body = SatelliteResponse)
    ),
    tag = "Satellites"
)]
pub async fn post_satellite(Json(req): Json<CreateSatelliteRequest>) -> Json<SatelliteResponse> {
    let satellite = create_satellite(req).await.unwrap();
    Json(satellite)
}

#[utoipa::path(
    put,
    path = "/api/v1/nexus/satellites/{id}/status",
    request_body = UpdateSatelliteStatusRequest,
    params(
        ("id" = Uuid, Path, description = "Satellite ID")
    ),
    responses(
        (status = 200, description = "Satellite status updated", body = SatelliteResponse)
    ),
    tag = "Satellites"
)]
pub async fn put_satellite_status(
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateSatelliteStatusRequest>,
) -> Json<SatelliteResponse> {
    let satellite = update_satellite_status(id, req).await.unwrap();
    Json(satellite)
}

#[utoipa::path(
    post,
    path = "/api/v1/nexus/satellites/position",
    request_body = CalculatePositionRequest,
    responses(
        (status = 200, description = "Flight positions calculated", body = CalculatePositionResponse)
    ),
    tag = "Satellites"
)]
pub async fn post_calculate_position(
    Json(req): Json<CalculatePositionRequest>,
) -> Json<CalculatePositionResponse> {
    let resp = calculate_position(req).await.unwrap();
    Json(resp)
}
