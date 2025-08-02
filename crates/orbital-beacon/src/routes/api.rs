use crate::models::FlightPositionRequest;
use crate::services::{SatelliteService, SatelliteServiceError};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde_json::json;
use sky_tracer::protocol::satellite::{
    CalculatePositionRequest, CalculatePositionResponse, CreateSatelliteRequest, SatelliteResponse,
    UpdateSatelliteStatusRequest,
};
use sky_tracer::protocol::{
    SATELLITES_API_PATH, SATELLITES_POSITION_API_PATH, SATELLITES_STATUS_API_PATH,
};
use tracing::{error, info, instrument, warn};
use uuid::Uuid;

/// Create a new satellite
#[utoipa::path(
    post,
    path = SATELLITES_API_PATH,
    request_body = CreateSatelliteRequest,
    responses(
        (status = 201, description = "Satellite created successfully", body = SatelliteResponse),
        (status = 400, description = "Invalid satellite data"),
        (status = 500, description = "Internal server error")
    ),
    tag = "satellites"
)]
#[instrument(skip(service), fields(satellite_name = %request.name))]
pub async fn create_satellite(
    State(service): State<SatelliteService>,
    Json(request): Json<CreateSatelliteRequest>,
) -> Result<(StatusCode, Json<SatelliteResponse>), (StatusCode, Json<serde_json::Value>)> {
    info!("Creating new satellite via API");

    match service.create_satellite(request.name).await {
        Ok(satellite) => {
            info!(
                satellite_id = %satellite.id,
                satellite_name = %satellite.name,
                status = ?satellite.status,
                "Satellite created successfully"
            );

            let response = SatelliteResponse {
                id: satellite.id,
                name: satellite.name,
                status: satellite.status,
            };
            Ok((StatusCode::CREATED, Json(response)))
        }
        Err(e) => {
            error!(error = %e, "Failed to create satellite");
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("Failed to create satellite: {}", e) })),
            ))
        }
    }
}

/// Update satellite status
#[utoipa::path(
    put,
    path = SATELLITES_STATUS_API_PATH,
    request_body = UpdateSatelliteStatusRequest,
    responses(
        (status = 200, description = "Satellite status updated", body = SatelliteResponse),
        (status = 404, description = "Satellite not found"),
        (status = 500, description = "Internal server error")
    ),
    params(
        ("id" = Uuid, Path, description = "Satellite ID")
    ),
    tag = "satellites"
)]
#[instrument(skip(service), fields(satellite_id = %id, new_status = ?request.status))]
pub async fn update_satellite_status(
    State(service): State<SatelliteService>,
    Path(id): Path<Uuid>,
    Json(request): Json<UpdateSatelliteStatusRequest>,
) -> Result<Json<SatelliteResponse>, (StatusCode, Json<serde_json::Value>)> {
    info!("Updating satellite status via API");

    match service.update_satellite_status(id, request.status).await {
        Ok(satellite) => {
            info!(
                satellite_id = %satellite.id,
                satellite_name = %satellite.name,
                status = ?satellite.status,
                "Satellite status updated successfully"
            );

            let response = SatelliteResponse {
                id: satellite.id,
                name: satellite.name,
                status: satellite.status,
            };
            Ok(Json(response))
        }
        Err(SatelliteServiceError::InvalidSatelliteId(_)) => {
            warn!(satellite_id = %id, "Satellite not found for status update");
            Err((
                StatusCode::NOT_FOUND,
                Json(json!({ "error": format!("Satellite not found: {}", id) })),
            ))
        }
        Err(e) => {
            error!(satellite_id = %id, error = %e, "Failed to update satellite status");
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("Failed to update satellite status: {}", e) })),
            ))
        }
    }
}

/// List all satellites
#[utoipa::path(
    get,
    path = SATELLITES_API_PATH,
    responses(
        (status = 200, description = "List of satellites", body = Vec<SatelliteResponse>),
        (status = 500, description = "Internal server error")
    ),
    tag = "satellites"
)]
#[instrument(skip(service))]
pub async fn list_satellites(
    State(service): State<SatelliteService>,
) -> Json<Vec<SatelliteResponse>> {
    info!("Listing all satellites via API");

    let satellites = service.list_satellites().await;

    let response: Vec<SatelliteResponse> = satellites
        .into_iter()
        .map(|s| SatelliteResponse {
            id: s.id,
            name: s.name,
            status: s.status,
        })
        .collect();

    info!(
        satellites_count = response.len(),
        "Retrieved satellite list via API"
    );
    Json(response)
}

/// Calculate flight position
#[utoipa::path(
    post,
    path = SATELLITES_POSITION_API_PATH,
    request_body = CalculatePositionRequest,
    responses(
        (status = 200, description = "Flight positions calculated", body = CalculatePositionResponse),
        (status = 400, description = "Invalid request data"),
        (status = 404, description = "Airport or satellites not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "satellites"
)]
#[instrument(skip(service), fields(
    departure = %request.departure,
    arrival = %request.arrival,
    departure_time = %request.departure_time,
    arrival_time = %request.arrival_time,
    current_time = ?request.current_time
))]
pub async fn calculate_position(
    State(service): State<SatelliteService>,
    Json(request): Json<CalculatePositionRequest>,
) -> Result<Json<CalculatePositionResponse>, (StatusCode, Json<serde_json::Value>)> {
    info!("Processing position calculation request via API");

    let flight_request = FlightPositionRequest::new(
        request.departure,
        request.arrival,
        request.departure_time,
        request.arrival_time,
    );

    let flight_request = if let Some(current_time) = request.current_time {
        flight_request.with_current_time(current_time)
    } else {
        flight_request
    };

    match service.calculate_flight_position(flight_request).await {
        Ok(calculation) => {
            info!(
                positions_count = calculation.positions.len(),
                has_departure = calculation.departure_airport.is_some(),
                has_arrival = calculation.arrival_airport.is_some(),
                "Position calculation successful"
            );

            let departure_airport_response = calculation
                .departure_airport
                .map(|airport| sky_tracer::protocol::airports::AirportResponse::from(&airport));
            let arrival_airport_response = calculation
                .arrival_airport
                .map(|airport| sky_tracer::protocol::airports::AirportResponse::from(&airport));

            let response = CalculatePositionResponse {
                positions: calculation.positions,
                departure_airport: departure_airport_response,
                arrival_airport: arrival_airport_response,
            };

            Ok(Json(response))
        }
        Err(SatelliteServiceError::NoActiveSatellites) => {
            warn!("No active satellites available for tracking");
            Err((
                StatusCode::NOT_FOUND,
                Json(json!({ "error": "No active satellites available for tracking" })),
            ))
        }
        Err(SatelliteServiceError::AirportNotFound(code)) => {
            warn!(airport_code = %code, "Airport not found");
            Err((
                StatusCode::NOT_FOUND,
                Json(json!({ "error": format!("Airport not found: {}", code) })),
            ))
        }
        Err(SatelliteServiceError::AirportFetchError(e)) => {
            error!(error = %e, "Failed to fetch airport data");
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("Failed to fetch airport data: {}", e) })),
            ))
        }
        Err(e) => {
            error!(error = %e, "Failed to calculate flight position");
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("Failed to calculate position: {}", e) })),
            ))
        }
    }
}
