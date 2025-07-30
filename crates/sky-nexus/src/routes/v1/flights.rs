use crate::services::flights::{create_flight, fetch_flight_by_number, fetch_flights};
use axum::{
    Json, Router,
    routing::{get, post},
};
use sky_tracer::model::flight::Flight;
use sky_tracer::protocol::flights::{CreateFlightRequest, FlightResponse};

pub fn router() -> Router {
    Router::new()
        .route("/", get(list_flights).post(post_flight))
        .route("/{flight_number}", get(get_flight))
}

#[utoipa::path(
    get,
    path = "/api/v1/flights",
    responses(
        (status = 200, description = "List all flights", body = [FlightResponse])
    ),
    tag = "Flights"
)]
pub async fn list_flights() -> Json<Vec<FlightResponse>> {
    let flights = fetch_flights().await.unwrap_or_default();
    let responses = flights
        .into_iter()
        .map(|f| FlightResponse {
            flight_number: f.flight_number,
            aircraft_number: f.aircraft_number,
            departure: f.departure,
            arrival: f.arrival,
            departure_time: f.departure_time,
            arrival_time: f.arrival_time,
        })
        .collect();
    Json(responses)
}

#[utoipa::path(
    post,
    path = "/api/v1/flights",
    request_body = CreateFlightRequest,
    responses(
        (status = 200, description = "Flight created", body = FlightResponse)
    ),
    tag = "Flights"
)]
pub async fn post_flight(Json(flight): Json<Flight>) -> Json<FlightResponse> {
    let created = create_flight(flight).await.unwrap(); // TODO: handle errors
    Json(FlightResponse {
        flight_number: created.flight_number,
        aircraft_number: created.aircraft_number,
        departure: created.departure,
        arrival: created.arrival,
        departure_time: created.departure_time,
        arrival_time: created.arrival_time,
    })
}

#[utoipa::path(
    get,
    path = "/api/v1/flights/{flight_number}",
    params(
        ("flight_number" = String, Path, description = "Flight number")
    ),
    responses(
        (status = 200, description = "Get flight by number", body = FlightResponse)
    ),
    tag = "Flights"
)]
pub async fn get_flight(
    axum::extract::Path(flight_number): axum::extract::Path<String>,
) -> Json<Option<FlightResponse>> {
    let flight = fetch_flight_by_number(&flight_number).await.ok();
    Json(flight.map(|f| FlightResponse {
        flight_number: f.flight_number,
        aircraft_number: f.aircraft_number,
        departure: f.departure,
        arrival: f.arrival,
        departure_time: f.departure_time,
        arrival_time: f.arrival_time,
    }))
}
