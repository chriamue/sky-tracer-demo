use crate::satellite_service::{SatelliteService, SatelliteServiceError};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;
use sky_tracer::protocol::satellite::{
    CalculatePositionRequest, CalculatePositionResponse, CreateSatelliteRequest, SatelliteResponse,
    UpdateSatelliteStatusRequest,
};
use tracing::instrument;
use uuid::Uuid;

/// Create a new satellite
#[utoipa::path(
    post,
    path = "/api/satellites",
    request_body = CreateSatelliteRequest,
    responses(
        (status = 201, description = "Satellite created successfully", body = SatelliteResponse),
        (status = 400, description = "Invalid satellite data")
    ),
    tag = "satellites"
)]
#[instrument(skip(service))]
pub async fn create_satellite(
    State(service): State<SatelliteService>,
    Json(request): Json<CreateSatelliteRequest>,
) -> impl IntoResponse {
    let satellite = service.create_satellite(request.name).await;
    let response = SatelliteResponse {
        id: satellite.id,
        name: satellite.name,
        status: satellite.status,
    };
    (StatusCode::CREATED, Json(response))
}

/// Update satellite status
#[utoipa::path(
    put,
    path = "/api/satellites/{id}/status",
    request_body = UpdateSatelliteStatusRequest,
    responses(
        (status = 200, description = "Satellite status updated", body = SatelliteResponse),
        (status = 404, description = "Satellite not found")
    ),
    params(
        ("id" = Uuid, Path, description = "Satellite ID")
    ),
    tag = "satellites"
)]
#[instrument(skip(service))]
pub async fn update_satellite_status(
    State(service): State<SatelliteService>,
    Path(id): Path<Uuid>,
    Json(request): Json<UpdateSatelliteStatusRequest>,
) -> impl IntoResponse {
    match service.update_status(id, request.status).await {
        Some(satellite) => {
            let response = SatelliteResponse {
                id: satellite.id,
                name: satellite.name,
                status: satellite.status,
            };
            Ok((StatusCode::OK, Json(response)))
        }
        None => Err((
            StatusCode::NOT_FOUND,
            Json(SatelliteResponse {
                id,
                name: String::new(),
                status: sky_tracer::model::SatelliteStatus::Inactive,
            }),
        )),
    }
}

/// List all satellites
#[utoipa::path(
    get,
    path = "/api/satellites",
    responses(
        (status = 200, description = "List of satellites", body = Vec<SatelliteResponse>)
    ),
    tag = "satellites"
)]
#[instrument(skip(service))]
pub async fn list_satellites(State(service): State<SatelliteService>) -> impl IntoResponse {
    let satellites = service.list_satellites().await;
    let response: Vec<SatelliteResponse> = satellites
        .into_iter()
        .map(|s| SatelliteResponse {
            id: s.id,
            name: s.name,
            status: s.status,
        })
        .collect();
    Json(response)
}

/// Calculate flight position
#[utoipa::path(
    post,
    path = "/api/position",
    request_body = CalculatePositionRequest,
    responses(
        (status = 200, description = "Flight positions calculated", body = CalculatePositionResponse),
        (status = 400, description = "Invalid request data")
    ),
    tag = "satellites"
)]
pub async fn calculate_position(
    State(service): State<SatelliteService>,
    Json(request): Json<CalculatePositionRequest>,
) -> impl IntoResponse {
    match service
        .calculate_position(
            &request.departure,
            &request.arrival,
            request.departure_time,
            request.arrival_time,
            request.current_time,
        )
        .await
    {
        Ok((positions, departure_airport, arrival_airport)) => {
            let departure_airport_response = departure_airport
                .map(|airport| sky_tracer::protocol::airports::AirportResponse::from(&airport));
            let arrival_airport_response = arrival_airport
                .map(|airport| sky_tracer::protocol::airports::AirportResponse::from(&airport));

            (
                StatusCode::OK,
                Json(CalculatePositionResponse {
                    positions,
                    departure_airport: departure_airport_response,
                    arrival_airport: arrival_airport_response,
                }),
            )
                .into_response()
        }
        Err(SatelliteServiceError::NoActiveSatellites) => (
            StatusCode::NOT_FOUND,
            Json(json!({
                "error": "No active satellites available for tracking"
            })),
        )
            .into_response(),
        Err(SatelliteServiceError::AirportNotFound(code)) => (
            StatusCode::NOT_FOUND,
            Json(json!({
                "error": format!("Airport not found: {}", code)
            })),
        )
            .into_response(),
        Err(SatelliteServiceError::AirportFetchError(e)) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": format!("Failed to fetch airport data: {}", e)
            })),
        )
            .into_response(),
    }
}
