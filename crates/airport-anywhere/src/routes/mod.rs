use axum::{extract::Query, Json};
use sky_tracer::protocol::airports::{SearchAirportsRequest, SearchAirportsResponse};
use sky_tracer::protocol::{AIRPORTS_API_PATH, AIRPORTS_SEARCH_API_PATH};
use tracing::{error, info, instrument};

use crate::services::AirportService;

/// List all airports
#[utoipa::path(
    get,
    path = AIRPORTS_API_PATH,
    responses(
        (status = 200, description = "List of all airports", body = SearchAirportsResponse),
        (status = 500, description = "Internal server error")
    ),
    tag = "airports"
)]
pub async fn list_airports() -> Json<SearchAirportsResponse> {
    match AirportService::get_all_airports().await {
        Ok(airports) => Json(SearchAirportsResponse { airports }),
        Err(e) => {
            error!(error = %e, "Failed to list airports");
            Json(SearchAirportsResponse { airports: vec![] })
        }
    }
}

/// Search airports by name or code
#[utoipa::path(
    get,
    path = AIRPORTS_SEARCH_API_PATH,
    params(
        SearchAirportsRequest
    ),
    responses(
        (status = 200, description = "List of matching airports", body = SearchAirportsResponse),
        (status = 500, description = "Internal server error")
    ),
    tag = "airports"
)]
#[instrument]
pub async fn search_airports(
    Query(params): Query<SearchAirportsRequest>,
) -> Json<SearchAirportsResponse> {
    info!(
        code = params.code.as_deref().unwrap_or("none"),
        name = params.name.as_deref().unwrap_or("none"),
        "Searching for airports"
    );

    match AirportService::search(params.code, params.name).await {
        Ok(airports) => Json(SearchAirportsResponse { airports }),
        Err(e) => {
            error!(error = %e, "Failed to search airports");
            Json(SearchAirportsResponse { airports: vec![] })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::extract::Query;

    #[tokio::test]
    async fn test_list_airports() {
        let response = list_airports().await;
        assert!(!response.airports.is_empty());
    }

    #[tokio::test]
    async fn test_search_airports_by_code() {
        let params = SearchAirportsRequest {
            name: None,
            code: Some("FRA".to_string()),
        };
        let response = search_airports(Query(params)).await;
        assert!(!response.airports.is_empty());
    }

    #[tokio::test]
    async fn test_search_airports_by_name() {
        let params = SearchAirportsRequest {
            name: Some("Frankfurt".to_string()),
            code: None,
        };
        let response = search_airports(Query(params)).await;
        assert!(!response.airports.is_empty());
        assert!(response
            .airports
            .iter()
            .any(|a| a.name.contains("Frankfurt")));
    }

    #[tokio::test]
    async fn test_search_airports_no_results() {
        let params = SearchAirportsRequest {
            name: None,
            code: Some("NONEXISTENT".to_string()),
        };
        let response = search_airports(Query(params)).await;
        assert!(response.airports.is_empty());
    }
}
