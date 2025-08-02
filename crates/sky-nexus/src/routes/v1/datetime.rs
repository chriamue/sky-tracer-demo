use crate::models::datetime::*;
use crate::services::datetime::{
    DateTimeServiceError, compare_timezones, get_aviation_times, get_current_datetime,
};
use axum::{Json, Router, extract::Query, http::StatusCode, response::IntoResponse, routing::get};
use tracing::{error, info, instrument};

pub fn router() -> Router {
    Router::new()
        .route("/current", get(get_current_datetime_route))
        .route("/aviation-times", get(get_aviation_times_route))
        .route("/compare", axum::routing::post(compare_timezones_route))
}

#[utoipa::path(
    get,
    path = "/api/v1/nexus/datetime/current",
    params(GetDateTimeQuery),
    responses(
        (status = 200, description = "Current date and time", body = DateTimeResponse),
        (status = 400, description = "Invalid timezone or format")
    ),
    tag = "DateTime"
)]
#[instrument]
pub async fn get_current_datetime_route(
    Query(query): Query<GetDateTimeQuery>,
) -> impl IntoResponse {
    info!(
        "Getting current datetime with timezone: {:?}, format: {:?}",
        query.timezone, query.format
    );

    match get_current_datetime(query).await {
        Ok(response) => {
            info!("Successfully retrieved current datetime");
            (StatusCode::OK, Json(response)).into_response()
        }
        Err(DateTimeServiceError::InvalidTimezone(msg)) => {
            error!("Invalid timezone: {}", msg);
            (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({
                    "error": "Invalid timezone",
                    "details": msg
                })),
            )
                .into_response()
        }
        Err(e) => {
            error!("Failed to get current datetime: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Failed to get current datetime",
                    "details": e.to_string()
                })),
            )
                .into_response()
        }
    }
}

#[utoipa::path(
    get,
    path = "/api/v1/nexus/datetime/aviation-times",
    responses(
        (status = 200, description = "Current time in major aviation hubs", body = AviationTimesResponse)
    ),
    tag = "DateTime"
)]
#[instrument]
pub async fn get_aviation_times_route() -> impl IntoResponse {
    info!("Getting aviation times");

    match get_aviation_times().await {
        Ok(response) => {
            info!(
                "Successfully retrieved aviation times for {} locations",
                response.times.len()
            );
            (StatusCode::OK, Json(response)).into_response()
        }
        Err(e) => {
            error!("Failed to get aviation times: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Failed to get aviation times",
                    "details": e.to_string()
                })),
            )
                .into_response()
        }
    }
}

#[utoipa::path(
    post,
    path = "/api/v1/nexus/datetime/compare",
    request_body = TimezoneComparisonRequest,
    responses(
        (status = 200, description = "Timezone comparison result", body = TimezoneComparisonResponse),
        (status = 400, description = "Invalid timezone")
    ),
    tag = "DateTime"
)]
#[instrument]
pub async fn compare_timezones_route(
    Json(request): Json<TimezoneComparisonRequest>,
) -> impl IntoResponse {
    info!(
        "Comparing timezones: {} vs {}",
        request.from_timezone, request.to_timezone
    );

    match compare_timezones(request).await {
        Ok(response) => {
            info!("Successfully compared timezones");
            (StatusCode::OK, Json(response)).into_response()
        }
        Err(DateTimeServiceError::InvalidTimezone(msg)) => {
            error!("Invalid timezone in comparison: {}", msg);
            (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({
                    "error": "Invalid timezone",
                    "details": msg
                })),
            )
                .into_response()
        }
        Err(e) => {
            error!("Failed to compare timezones: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Failed to compare timezones",
                    "details": e.to_string()
                })),
            )
                .into_response()
        }
    }
}
