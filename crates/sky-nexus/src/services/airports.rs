use reqwest::Client;
use sky_tracer::model::airport::Airport;
use sky_tracer::protocol::airports::SearchAirportsResponse;
use std::env;
use tracing::{info, instrument};

fn get_airport_service_url() -> String {
    env::var("AIRPORT_SERVICE_URL").unwrap_or_else(|_| "http://localhost:3000".to_string())
}

#[instrument]
pub async fn fetch_airports() -> Result<Vec<Airport>, reqwest::Error> {
    info!("Fetching all airports");
    let client = Client::new();
    let base_url = get_airport_service_url();
    let url = format!("{}/api/airports", base_url);

    info!(url = %url, "Making request to fetch airports");
    let resp = client.get(&url).send().await?;

    if resp.status().is_success() {
        let search_response = resp.json::<SearchAirportsResponse>().await?;
        let airports: Vec<Airport> = search_response
            .airports
            .into_iter()
            .map(|airport_response| Airport {
                id: airport_response.id,
                latitude: airport_response.position.latitude,
                longitude: airport_response.position.longitude,
                name: airport_response.name,
                code: airport_response.code,
            })
            .collect();

        info!(count = airports.len(), "Successfully fetched airports");
        Ok(airports)
    } else {
        info!(status = %resp.status(), "Failed to fetch airports");
        Err(reqwest::Error::from(resp.error_for_status().unwrap_err()))
    }
}

#[instrument]
pub async fn fetch_airport_by_code(code: &str) -> Result<Airport, reqwest::Error> {
    info!(code = %code, "Fetching airport by code");
    let client = Client::new();
    let base_url = get_airport_service_url();
    let url = format!("{}/api/airports/search?code={}", base_url, code);

    info!(url = %url, "Making request to search airport");
    let resp = client.get(&url).send().await?;

    if resp.status().is_success() {
        let search_response = resp.json::<SearchAirportsResponse>().await?;

        if let Some(airport_response) = search_response.airports.first() {
            let airport = Airport {
                id: airport_response.id,
                latitude: airport_response.position.latitude,
                longitude: airport_response.position.longitude,
                name: airport_response.name.clone(),
                code: airport_response.code.clone(),
            };

            info!(code = %code, name = %airport.name, "Successfully found airport");
            Ok(airport)
        } else {
            info!(code = %code, "Airport not found");
            Err(reqwest::Error::from(
                reqwest::Client::new()
                    .get("http://example.com/not-found")
                    .send()
                    .await
                    .unwrap()
                    .error_for_status()
                    .unwrap_err(),
            ))
        }
    } else {
        info!(status = %resp.status(), code = %code, "Failed to search airport");
        Err(reqwest::Error::from(resp.error_for_status().unwrap_err()))
    }
}
