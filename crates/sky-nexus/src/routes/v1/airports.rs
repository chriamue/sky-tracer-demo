use crate::services::airports::{AirportServiceError, fetch_airport_by_code, fetch_airports};
use axum::{Json, Router, extract::Path, http::StatusCode, response::IntoResponse, routing::get};
use sky_tracer::protocol::{AIRPORTS_API_PATH, airports::AirportResponse};
use tracing::{error, info, instrument};

pub fn router() -> Router {
    Router::new()
        .route("/", get(list_airports))
        .route("/{code}", get(get_airport))
}

#[utoipa::path(
    get,
    path = AIRPORTS_API_PATH,
    responses(
        (status = 200, description = "List all airports", body = [AirportResponse]),
        (status = 500, description = "Internal server error")
    ),
    tag = "Airports"
)]
#[instrument]
pub async fn list_airports() -> impl IntoResponse {
    info!("Listing airports");

    match fetch_airports().await {
        Ok(airports) => {
            let responses: Vec<AirportResponse> =
                airports.iter().map(AirportResponse::from).collect();
            info!("Successfully listed {} airports", responses.len());
            (StatusCode::OK, Json(responses)).into_response()
        }
        Err(e) => {
            error!("Failed to fetch airports: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Failed to fetch airports",
                    "details": e.to_string()
                })),
            )
                .into_response()
        }
    }
}

#[utoipa::path(
    get,
    path = "/api/v1/airports/{code}",
    params(
        ("code" = String, Path, description = "Airport code")
    ),
    responses(
        (status = 200, description = "Get airport by code", body = AirportResponse),
        (status = 404, description = "Airport not found")
    ),
    tag = "Airports"
)]
#[instrument]
pub async fn get_airport(Path(code): Path<String>) -> impl IntoResponse {
    info!("Getting airport by code: {}", code);

    match fetch_airport_by_code(&code).await {
        Ok(airport) => {
            let response = AirportResponse::from(&airport);
            (StatusCode::OK, Json(response)).into_response()
        }
        Err(AirportServiceError::NotFound(_)) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({
                "error": "Airport not found",
                "code": code
            })),
        )
            .into_response(),
        Err(e) => {
            error!("Failed to get airport {}: {}", code, e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Failed to get airport",
                    "details": e.to_string()
                })),
            )
                .into_response()
        }
    }
}
