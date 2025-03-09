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
#[instrument(
    skip(service),
    fields(
        satellite_name = %request.name
    )
)]
pub async fn create_satellite(
    State(service): State<SatelliteService>,
    Json(request): Json<CreateSatelliteRequest>,
) -> impl IntoResponse {
    use tracing::{debug, info};

    debug!("Creating new satellite");
    let satellite = service.create_satellite(request.name).await;

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
#[instrument(
    skip(service),
    fields(
        satellite_id = %id,
        new_status = ?request.status
    )
)]
pub async fn update_satellite_status(
    State(service): State<SatelliteService>,
    Path(id): Path<Uuid>,
    Json(request): Json<UpdateSatelliteStatusRequest>,
) -> impl IntoResponse {
    use tracing::{debug, info, warn};

    debug!("Updating satellite status");

    match service.update_status(id, request.status).await {
        Some(satellite) => {
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
            Ok((StatusCode::OK, Json(response)))
        }
        None => {
            warn!(satellite_id = %id, "Satellite not found");
            Err((
                StatusCode::NOT_FOUND,
                Json(SatelliteResponse {
                    id,
                    name: String::new(),
                    status: sky_tracer::model::SatelliteStatus::Inactive,
                }),
            ))
        }
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
    use tracing::{debug, info};

    debug!("Listing all satellites");
    let satellites = service.list_satellites().await;

    let active_count = satellites.iter().filter(|s| s.is_active()).count();
    info!(
        total_satellites = satellites.len(),
        active_satellites = active_count,
        "Retrieved satellite list"
    );

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
#[instrument(
    skip(service),
    fields(
        departure = %request.departure,
        arrival = %request.arrival,
        departure_time = %request.departure_time,
        arrival_time = ?request.arrival_time,
        current_time = ?request.current_time
    )
)]
pub async fn calculate_position(
    State(service): State<SatelliteService>,
    Json(request): Json<CalculatePositionRequest>,
) -> impl IntoResponse {
    use tracing::{debug, error, info, warn};

    debug!("Processing position calculation request");

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
            let positions_count = positions.len();

            if positions.is_empty() {
                warn!("No positions calculated for flight");
            } else {
                debug!(
                    first_position_lat = positions[0].latitude,
                    first_position_lon = positions[0].longitude,
                    first_position_alt = positions[0].altitude,
                    "Calculated flight position"
                );
            }

            // Log airport information
            if let Some(ref dep) = departure_airport {
                debug!(
                    airport = "departure",
                    code = %dep.code,
                    name = %dep.name,
                    lat = dep.latitude,
                    lon = dep.longitude,
                    "Departure airport details"
                );
            } else {
                warn!(airport = %request.departure, "Departure airport not found");
            }

            if let Some(ref arr) = arrival_airport {
                debug!(
                    airport = "arrival",
                    code = %arr.code,
                    name = %arr.name,
                    lat = arr.latitude,
                    lon = arr.longitude,
                    "Arrival airport details"
                );
            } else {
                warn!(airport = %request.arrival, "Arrival airport not found");
            }

            info!(
                positions_count = positions_count,
                has_departure = departure_airport.is_some(),
                has_arrival = arrival_airport.is_some(),
                "Position calculation successful"
            );

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
        Err(SatelliteServiceError::NoActiveSatellites) => {
            warn!("No active satellites available for tracking");
            (
                StatusCode::NOT_FOUND,
                Json(json!({
                    "error": "No active satellites available for tracking"
                })),
            )
                .into_response()
        }
        Err(SatelliteServiceError::AirportNotFound(code)) => {
            warn!(airport_code = %code, "Airport not found");
            (
                StatusCode::NOT_FOUND,
                Json(json!({
                    "error": format!("Airport not found: {}", code)
                })),
            )
                .into_response()
        }
        Err(SatelliteServiceError::AirportFetchError(e)) => {
            error!(
                error = %e,
                "Failed to fetch airport data"
            );
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": format!("Failed to fetch airport data: {}", e)
                })),
            )
                .into_response()
        }
    }
}
