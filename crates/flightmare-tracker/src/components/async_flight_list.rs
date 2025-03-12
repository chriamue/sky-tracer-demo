use gloo_net::http::Request;
use gloo_timers::future::sleep;
use sky_tracer::protocol::flights::FlightResponse;
use std::time::Duration;
use yew::prelude::*;
use yew::suspense::use_future;

use crate::components::GrundDisplay;
use crate::grund::{get_random_grund, Grund};

#[derive(Clone, PartialEq)]
struct FlightWithDelay {
    flight: FlightResponse,
    grund: Option<Grund>,
}

async fn fetch_flights() -> Result<Vec<FlightWithDelay>, String> {
    sleep(Duration::from_secs(5)).await;

    let response = Request::get("/api/flights")
        .send()
        .await
        .map_err(|e| format!("Failed to fetch flights: {}", e))?;

    let flights: Vec<FlightResponse> = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse flight data: {}", e))?;

    Ok(flights
        .into_iter()
        .map(|flight| FlightWithDelay {
            flight,
            grund: if rand::random::<f32>() < 0.5 {
                Some(get_random_grund())
            } else {
                None
            },
        })
        .collect())
}

#[function_component(FlightListContent)]
fn flight_list_content() -> HtmlResult {
    let flights = use_future(|| async move { fetch_flights().await })?;

    match &*flights {
        Ok(flights) => Ok(html! {
            <div class="flight-list">
                {flights.iter().map(|flight_with_delay| {
                    html! {
                        <div class="flight-item">
                            <div class="flight-info">
                                <h3>{&flight_with_delay.flight.flight_number}</h3>
                                <p class="route">
                                    {&flight_with_delay.flight.departure}
                                    {" → "}
                                    {&flight_with_delay.flight.arrival}
                                </p>
                                <p class="time">
                                    {"Departure: "}
                                    {flight_with_delay.flight.departure_time.format("%Y-%m-%d %H:%M")}
                                </p>
                                if let Some(arrival_time) = flight_with_delay.flight.arrival_time {
                                    <p class="time">
                                        {"Arrival: "}
                                        {arrival_time.format("%Y-%m-%d %H:%M")}
                                    </p>
                                }
                            </div>
                            <GrundDisplay grund={flight_with_delay.grund.clone()} />
                        </div>
                    }
                }).collect::<Html>()}
            </div>
        }),
        Err(error) => Ok(html! {
            <div class="error-message">
                <p>{error}</p>
            </div>
        }),
    }
}

#[function_component(AsyncFlightList)]
pub fn async_flight_list() -> Html {
    html! {
        <Suspense fallback={html! {
            <div class="loading">
                <div class="spinner">{"🌪️"}</div>
                <p>{"Finding excuses..."}</p>
            </div>
        }}>
            <FlightListContent />
        </Suspense>
    }
}
