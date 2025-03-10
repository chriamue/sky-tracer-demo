use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use sky_tracer::protocol::flights::FlightResponse;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::components::GrundDisplay;
use crate::grund::{get_random_grund, Grund};

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct FlightWithDelay {
    pub flight: FlightResponse,
    pub grund: Option<Grund>,
}

#[function_component(App)]
pub fn app() -> Html {
    let flights = use_state(Vec::<FlightWithDelay>::new);
    let loading = use_state(|| true);
    let error = use_state(|| Option::<String>::None);
    let error_grund = use_state(|| get_random_grund());

    {
        let flights = flights.clone();
        let loading = loading.clone();
        let error = error.clone();
        let error_grund = error_grund.clone();

        use_effect_with((), move |_| {
            spawn_local(async move {
                loading.set(true);
                match Request::get("/api/flights").send().await {
                    Ok(response) => match response.json::<Vec<FlightResponse>>().await {
                        Ok(flight_data) => {
                            let flights_with_delay: Vec<FlightWithDelay> = flight_data
                                .into_iter()
                                .map(|flight| FlightWithDelay {
                                    flight,
                                    grund: if rand::random::<f32>() < 0.3 {
                                        Some(get_random_grund())
                                    } else {
                                        None
                                    },
                                })
                                .collect();
                            flights.set(flights_with_delay);
                        }
                        Err(e) => {
                            error_grund.set(get_random_grund());
                            error.set(Some(format!("Failed to parse flight data: {}", e)))
                        }
                    },
                    Err(e) => {
                        error_grund.set(get_random_grund());
                        error.set(Some(format!("Failed to fetch flights: {}", e)))
                    }
                }
                loading.set(false);
            });
            || ()
        });
    }

    html! {
        <div class="container">
            <header>
                <h1>{"🛫 Flightmare Tracker"}</h1>
                <p class="subtitle">{"Your premier source for simulated flight delays!"}</p>
            </header>
            <main>
                if *loading {
                    <div class="loading">
                        <div class="spinner"></div>
                        <p>{"Loading flights..."}</p>
                    </div>
                } else if let Some(err) = &*error {
                    <div class="error-container">
                        <div class="error">
                            <h2>{"System Delay"}</h2>
                            <GrundDisplay grund={Some((*error_grund).clone())} />
                            <div class="technical-details">
                                <p>{"Technical Details:"}</p>
                                <code>{err}</code>
                            </div>
                        </div>
                        <button onclick={
                            let error_grund = error_grund.clone();
                            Callback::from(move |_| {
                                error_grund.set(get_random_grund());
                            })
                        }>
                            {"Get Another Reason"}
                        </button>
                    </div>
                } else {
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
                }
            </main>
        </div>
    }
}
