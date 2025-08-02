use crate::models::AirportsService;
use sky_tracer::model::airport::AirportError;
use sky_tracer::protocol::airports::AirportResponse;
use tracing::{info, instrument};

/// Airport service for business logic operations
pub struct AirportService;

impl AirportService {
    /// Get all airports
    #[instrument]
    pub async fn get_all_airports() -> Result<Vec<AirportResponse>, AirportError> {
        let service = AirportsService::instance()?;
        let airports: Vec<AirportResponse> = service
            .all()
            .map(|airport| AirportResponse::from(airport.as_ref()))
            .collect();

        info!(count = airports.len(), "Retrieved all airports");
        Ok(airports)
    }

    /// Search airports by code (IATA or ICAO)
    #[instrument]
    pub async fn search_by_code(code: &str) -> Result<Vec<AirportResponse>, AirportError> {
        let service = AirportsService::instance()?;

        match service.find_by_code(code) {
            Ok(airport) => {
                info!(code = %code, "Found airport by code");
                Ok(vec![AirportResponse::from(airport.as_ref())])
            }
            Err(AirportError::NotFound(_)) => {
                info!(code = %code, "No airport found with code");
                Ok(vec![])
            }
            Err(e) => {
                tracing::error!(code = %code, error = %e, "Error searching airport by code");
                Err(e)
            }
        }
    }

    /// Search airports by name (partial match)
    #[instrument]
    pub async fn search_by_name(name_query: &str) -> Result<Vec<AirportResponse>, AirportError> {
        let service = AirportsService::instance()?;
        let airports: Vec<AirportResponse> = service
            .search_by_name(name_query)
            .into_iter()
            .map(|airport| AirportResponse::from(airport.as_ref()))
            .collect();

        info!(query = %name_query, count = airports.len(), "Searched airports by name");
        Ok(airports)
    }

    /// Combined search - try code first, then name
    #[instrument]
    pub async fn search(
        code: Option<String>,
        name: Option<String>,
    ) -> Result<Vec<AirportResponse>, AirportError> {
        if let Some(code) = code {
            Self::search_by_code(&code).await
        } else if let Some(name_query) = name {
            Self::search_by_name(&name_query).await
        } else {
            Self::get_all_airports().await
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_all_airports() {
        let result = AirportService::get_all_airports().await;
        assert!(result.is_ok());
        let airports = result.unwrap();
        assert!(!airports.is_empty());
    }

    #[tokio::test]
    async fn test_search_by_code() {
        let result = AirportService::search_by_code("FRA").await;
        assert!(result.is_ok());
        let airports = result.unwrap();
        assert!(!airports.is_empty());
        assert!(airports.iter().any(|a| a.code.contains("FRA")));
    }

    #[tokio::test]
    async fn test_search_by_name() {
        let result = AirportService::search_by_name("Frankfurt").await;
        assert!(result.is_ok());
        let airports = result.unwrap();
        assert!(!airports.is_empty());
        assert!(airports.iter().any(|a| a.name.contains("Frankfurt")));
    }

    #[tokio::test]
    async fn test_search_nonexistent_code() {
        let result = AirportService::search_by_code("NONEXISTENT").await;
        assert!(result.is_ok());
        let airports = result.unwrap();
        assert!(airports.is_empty());
    }

    #[tokio::test]
    async fn test_combined_search_with_code() {
        let result = AirportService::search(Some("FRA".to_string()), None).await;
        assert!(result.is_ok());
        let airports = result.unwrap();
        assert!(!airports.is_empty());
    }

    #[tokio::test]
    async fn test_combined_search_with_name() {
        let result = AirportService::search(None, Some("Frankfurt".to_string())).await;
        assert!(result.is_ok());
        let airports = result.unwrap();
        assert!(!airports.is_empty());
    }

    #[tokio::test]
    async fn test_combined_search_all() {
        let result = AirportService::search(None, None).await;
        assert!(result.is_ok());
        let airports = result.unwrap();
        assert!(!airports.is_empty());
    }
}
