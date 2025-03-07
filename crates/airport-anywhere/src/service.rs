use axum::{extract::Query, Json};
use sky_tracer::protocol::airports::{
    AirportResponse, SearchAirportsRequest, SearchAirportsResponse,
};

use crate::airports_service::AirportsService;

/// List all airports
#[utoipa::path(
    get,
    path = "/api/airports",
    responses(
        (status = 200, description = "List of all airports", body = SearchAirportsResponse),
        (status = 500, description = "Internal server error")
    ),
    tag = "airports"
)]
pub async fn list_airports() -> Json<SearchAirportsResponse> {
    match AirportsService::instance() {
        Ok(service) => {
            let airports: Vec<AirportResponse> = service
                .all()
                .map(|airport| AirportResponse::from(airport.as_ref()))
                .collect();

            Json(SearchAirportsResponse { airports })
        }
        Err(_) => Json(SearchAirportsResponse { airports: vec![] }),
    }
}

/// Search airports by name or code
#[utoipa::path(
    get,
    path = "/api/airports/search",
    params(
        SearchAirportsRequest
    ),
    responses(
        (status = 200, description = "List of matching airports", body = SearchAirportsResponse),
        (status = 500, description = "Internal server error")
    ),
    tag = "airports"
)]
pub async fn search_airports(
    Query(params): Query<SearchAirportsRequest>,
) -> Json<SearchAirportsResponse> {
    match AirportsService::instance() {
        Ok(service) => {
            let airports = if let Some(code) = params.code {
                if let Ok(airport) = service.find_by_code(&code) {
                    vec![AirportResponse::from(airport.as_ref())]
                } else {
                    vec![]
                }
            } else if let Some(name_query) = params.name {
                service
                    .search_by_name(&name_query)
                    .into_iter()
                    .map(|airport| AirportResponse::from(airport.as_ref()))
                    .collect()
            } else {
                service
                    .all()
                    .map(|airport| AirportResponse::from(airport.as_ref()))
                    .collect()
            };

            Json(SearchAirportsResponse { airports })
        }
        Err(_) => Json(SearchAirportsResponse { airports: vec![] }),
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
            code: Some("FRA/EDDF".to_string()),
        };
        let response = search_airports(Query(params)).await;

        assert_eq!(response.airports.len(), 1);
        assert_eq!(response.airports[0].name, "Frankfurt am Main Airport");
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
