use gloo_net::http::Request;
use gloo_timers::future::sleep;
use sky_tracer::protocol::flights::FlightResponse;
use std::time::Duration;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::components::GrundDisplay;
use crate::grund::{get_random_grund, Grund};

#[derive(Clone, PartialEq)]
pub struct FlightWithDelay {
    pub flight: FlightResponse,
    pub grund: Option<Grund>,
}

#[function_component(FlightList)]
pub fn flight_list() -> Html {
    let flights = use_state(Vec::<FlightWithDelay>::new);
    let loading = use_state(|| true);
    let error = use_state(|| Option::<String>::None);

    {
        let flights = flights.clone();
        let loading = loading.clone();
        let error = error.clone();

        use_effect_with(flights.clone(), move |_| {
            spawn_local(async move {
                sleep(Duration::from_secs(2)).await;
                loading.set(true);
                sleep(Duration::from_secs(2)).await;
                match Request::get("/api/flights").send().await {
                    Ok(response) => match response.json::<Vec<FlightResponse>>().await {
                        Ok(flight_data) => {
                            let flights_with_delay: Vec<FlightWithDelay> = flight_data
                                .into_iter()
                                .map(|flight| FlightWithDelay {
                                    flight,
                                    grund: if rand::random::<f32>() < 0.5 {
                                        Some(get_random_grund())
                                    } else {
                                        None
                                    },
                                })
                                .collect();
                            flights.set(flights_with_delay);
                            error.set(None);
                        }
                        Err(e) => error.set(Some(format!("Failed to parse flight data: {}", e))),
                    },
                    Err(e) => error.set(Some(format!("Failed to fetch flights: {}", e))),
                }
                loading.set(false);
            });
            || ()
        });
    }

    html! {
        <div class="flight-list">
            if *loading {
                <div class="loading">
                    <div class="loading-sequence">
                        <span>{"✈️"}</span>
                        <span>{"💨"}</span>
                        <span>{"🌥️"}</span>
                    </div>
                    <p>{"Finding excuses..."}</p>
                </div>
            } else if let Some(err) = &*error {
                <div class="error-message">
                    <p>{err}</p>
                </div>
            } else {
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
            }
        </div>
    }
}
