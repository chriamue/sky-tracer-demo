use gloo_net::http::Request;
use serde_json::json;
use sky_tracer::protocol::airports::AirportResponse;
use sky_tracer::protocol::flights::{FlightPositionResponse, FlightResponse};
use std::collections::HashMap;
use uuid::Uuid;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[derive(Clone)]
struct FlightData {
    flight: FlightResponse,
    position: Option<FlightPositionResponse>,
}

#[derive(Clone, Default)]
struct MapState {
    flights: HashMap<String, FlightData>,
    airports: HashMap<String, AirportResponse>,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window)]
    fn initializeFlightsMap(containerId: &str, mapData: &str);
}

#[function_component(FlightMap)]
pub fn flight_map() -> Html {
    let map_id = use_state(|| format!("map-{}", Uuid::new_v4()));
    let map_state = use_state(MapState::default);
    let loading = use_state(|| true);

    // Function to fetch flight positions
    let fetch_positions = {
        let map_state = map_state.clone();
        let map_id = map_id.clone();

        move || {
            let map_state = map_state.clone();
            let map_id = map_id.clone();
            let flight_numbers: Vec<String> = map_state.flights.keys().cloned().collect();

            for flight_number in flight_numbers {
                let map_state = map_state.clone();
                let map_id = map_id.clone();

                spawn_local(async move {
                    match Request::get(&format!("/api/flights/{}/position", flight_number))
                        .send()
                        .await
                    {
                        Ok(response) => {
                            if let Ok(position) = response.json::<FlightPositionResponse>().await {
                                map_state.set({
                                    let mut new_state = (*map_state).clone();
                                    if let Some(flight_data) =
                                        new_state.flights.get_mut(&flight_number)
                                    {
                                        flight_data.position = Some(position);
                                    }
                                    new_state
                                });

                                update_map(&map_id, &*map_state);
                            }
                        }
                        Err(err) => log::error!(
                            "Error fetching position for flight {}: {}",
                            flight_number,
                            err
                        ),
                    }
                });
            }
        }
    };

    // Function to fetch airport data
    let fetch_airport = {
        let map_state = map_state.clone();

        move |code: &str| {
            let code = code.to_string();
            let map_state = map_state.clone();

            spawn_local(async move {
                if !map_state.airports.contains_key(&code) {
                    match Request::get(&format!("/api/airports/search?code={}", code))
                        .send()
                        .await
                    {
                        Ok(response) => {
                            if let Ok(mut airports) = response.json::<Vec<AirportResponse>>().await
                            {
                                if let Some(airport) = airports.pop() {
                                    map_state.set({
                                        let mut new_state = (*map_state).clone();
                                        new_state.airports.insert(code, airport);
                                        new_state
                                    });
                                }
                            }
                        }
                        Err(err) => log::error!("Error fetching airport {}: {}", code, err),
                    }
                }
            });
        }
    };

    // Function to fetch flights data
    let fetch_flights = {
        let map_state = map_state.clone();
        let loading = loading.clone();
        let map_id = map_id.clone();
        let fetch_airport = fetch_airport.clone();

        move || {
            let map_state = map_state.clone();
            let loading = loading.clone();
            let map_id = map_id.clone();
            let fetch_airport = fetch_airport.clone();

            spawn_local(async move {
                match Request::get("/api/flights").send().await {
                    Ok(response) => {
                        if let Ok(flights) = response.json::<Vec<FlightResponse>>().await {
                            // Update flights in state
                            map_state.set({
                                let mut new_state = (*map_state).clone();
                                for flight in flights {
                                    // Fetch airport data if not already present
                                    fetch_airport(&flight.departure);
                                    fetch_airport(&flight.arrival);

                                    new_state.flights.insert(
                                        flight.flight_number.clone(),
                                        FlightData {
                                            flight,
                                            position: None,
                                        },
                                    );
                                }
                                new_state
                            });

                            update_map(&map_id, &*map_state);
                        }
                    }
                    Err(err) => log::error!("Error fetching flights: {}", err),
                }
                loading.set(false);
            });
        }
    };

    // Initial load and setup periodic refresh
    {
        let fetch_flights = fetch_flights.clone();
        let fetch_positions = fetch_positions.clone();

        use_effect_with((), move |_| {
            let fetch_flights = fetch_flights.clone();
            let fetch_positions = fetch_positions.clone();

            // Initial fetch
            fetch_flights();

            // Set up interval for flights
            let flights_interval = gloo_timers::callback::Interval::new(30_000, move || {
                fetch_flights();
            });

            // Set up separate fetch_positions clone for the second interval
            let fetch_positions2 = fetch_positions.clone();

            // Set up interval for positions
            let positions_interval = gloo_timers::callback::Interval::new(10_000, move || {
                fetch_positions2();
            });

            // Cleanup function
            move || {
                drop(flights_interval);
                drop(positions_interval);
            }
        });
    }

    html! {
        <div class="map-section">
            <h2>{"Live Flight Map"}</h2>
            if *loading {
                <div class="loading-overlay">
                    <span>{"Loading flights..."}</span>
                </div>
            }
            <div id={(*map_id).clone()}
                 class="map-container">
            </div>
        </div>
    }
}

fn update_map(map_id: &str, state: &MapState) {
    let map_data = json!({
        "flights": state.flights.values().map(|data| {
            let flight = &data.flight;
            json!({
                "flightNumber": flight.flight_number,
                "departure": {
                    "code": flight.departure,
                    "position": state.airports.get(&flight.departure).map(|a| [a.position.latitude, a.position.longitude])
                        .unwrap_or([50.033333, 8.570556])
                },
                "arrival": {
                    "code": flight.arrival,
                    "position": state.airports.get(&flight.arrival).map(|a| [a.position.latitude, a.position.longitude])
                        .unwrap_or([38.7223, -9.1393])
                },
                "position": data.position.as_ref().map(|pos| [pos.latitude, pos.longitude])
            })
        }).collect::<Vec<_>>()
    });

    initializeFlightsMap(map_id, &map_data.to_string());
}
