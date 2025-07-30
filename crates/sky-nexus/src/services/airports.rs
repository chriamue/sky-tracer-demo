use reqwest::Client;
use sky_tracer::model::airport::Airport;

const AIRPORT_ANYWHERE_URL: &str = "http://localhost:8001/api/v1/airports";

pub async fn fetch_airports() -> Result<Vec<Airport>, reqwest::Error> {
    let client = Client::new();
    let resp = client.get(AIRPORT_ANYWHERE_URL).send().await?;
    resp.json::<Vec<Airport>>().await
}

pub async fn fetch_airport_by_code(code: &str) -> Result<Airport, reqwest::Error> {
    let client = Client::new();
    let url = format!("{}/{}", AIRPORT_ANYWHERE_URL, code);
    let resp = client.get(&url).send().await?;
    resp.json::<Airport>().await
}
