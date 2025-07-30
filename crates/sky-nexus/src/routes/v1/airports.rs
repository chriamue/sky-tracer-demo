use crate::services::airports::{fetch_airport_by_code, fetch_airports};
use axum::{Json, Router, routing::get};
use sky_tracer::protocol::airports::AirportResponse;

pub fn router() -> Router {
    Router::new()
        .route("/", get(list_airports))
        .route("/{code}", get(get_airport))
}

#[utoipa::path(
    get,
    path = "/api/v1/airports",
    responses(
        (status = 200, description = "List all airports", body = [AirportResponse])
    ),
    tag = "Airports"
)]
pub async fn list_airports() -> Json<Vec<AirportResponse>> {
    let airports = fetch_airports().await.unwrap_or_default();
    let responses = airports.iter().map(AirportResponse::from).collect();
    Json(responses)
}

#[utoipa::path(
    get,
    path = "/api/v1/airports/{code}",
    params(
        ("code" = String, Path, description = "Airport code")
    ),
    responses(
        (status = 200, description = "Get airport by code", body = AirportResponse)
    ),
    tag = "Airports"
)]
pub async fn get_airport(
    axum::extract::Path(code): axum::extract::Path<String>,
) -> Json<Option<AirportResponse>> {
    let airport = fetch_airport_by_code(&code).await.ok();
    Json(airport.map(|a| AirportResponse::from(&a)))
}
