use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

use sky_tracer::protocol::flights::ErrorResponse;

#[derive(Debug)]
pub enum ApiError {
    RequestError(String),
    ParseError(String),
    NotFound(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_response) = match self {
            ApiError::RequestError(msg) => (
                StatusCode::BAD_REQUEST,
                ErrorResponse {
                    error: msg,
                    code: "REQUEST_ERROR".into(),
                },
            ),
            ApiError::ParseError(msg) => (
                StatusCode::BAD_REQUEST,
                ErrorResponse {
                    error: msg,
                    code: "PARSE_ERROR".into(),
                },
            ),
            ApiError::NotFound(msg) => (
                StatusCode::NOT_FOUND,
                ErrorResponse {
                    error: msg,
                    code: "NOT_FOUND".into(),
                },
            ),
        };

        (status, Json(error_response)).into_response()
    }
}
