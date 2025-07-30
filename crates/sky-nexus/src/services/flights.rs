use reqwest::Client;
use sky_tracer::model::flight::Flight;

const TOWER_OF_BABEL_URL: &str = "http://localhost:8002/api/v1/flights";

pub async fn fetch_flights() -> Result<Vec<Flight>, reqwest::Error> {
    let client = Client::new();
    let resp = client.get(TOWER_OF_BABEL_URL).send().await?;
    resp.json::<Vec<Flight>>().await
}

pub async fn fetch_flight_by_number(flight_number: &str) -> Result<Flight, reqwest::Error> {
    let client = Client::new();
    let url = format!("{}/{}", TOWER_OF_BABEL_URL, flight_number);
    let resp = client.get(&url).send().await?;
    resp.json::<Flight>().await
}

pub async fn create_flight(flight: Flight) -> Result<Flight, reqwest::Error> {
    let client = Client::new();
    let resp = client.post(TOWER_OF_BABEL_URL).json(&flight).send().await?;
    resp.json::<Flight>().await
}
