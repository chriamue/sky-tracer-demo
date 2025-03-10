use super::FlightMap;
use crate::types::{Airport, Flight};
use gloo_net::http::Request;
use log::{debug, info, warn};
use sky_tracer::protocol::flights::{FlightPositionResponse, FlightResponse};
use std::collections::HashMap;
use yew::prelude::*;

#[function_component(FlightTracker)]
pub fn flight_tracker() -> Html {
    let flights = use_state(Vec::<Flight>::new);
    let airports = use_state(Vec::<Airport>::new);
    let loading = use_state(|| true);

    // Fetch flights and airports
    {
        let flights = flights.clone();
        let airports = airports.clone();
        let loading = loading.clone();

        use_effect_with((), move |_| {
            let flights = flights.clone();
            let airports = airports.clone();
            let loading = loading.clone();

            async fn fetch_data(
                flights: UseStateHandle<Vec<Flight>>,
                airports: UseStateHandle<Vec<Airport>>,
                loading: UseStateHandle<bool>,
            ) {
                let mut airport_map = HashMap::new();

                // Fetch flights
                debug!("Fetching flights...");
                match Request::get("/api/flights").send().await {
                    Ok(response) => {
                        match response.json::<Vec<FlightResponse>>().await {
                            Ok(flight_list) => {
                                info!("Received {} flights", flight_list.len());

                                // Collect unique airport codes
                                let airport_codes: Vec<String> = flight_list
                                    .iter()
                                    .flat_map(|f| vec![f.departure.clone(), f.arrival.clone()])
                                    .collect::<std::collections::HashSet<_>>()
                                    .into_iter()
                                    .collect();

                                // Fetch all airport data first
                                for code in airport_codes {
                                    match Request::get(&format!(
                                        "/api/airports/search?code={}",
                                        code
                                    ))
                                    .send()
                                    .await
                                    {
                                        Ok(response) => {
                                            if let Ok(search_response) = response
                                                .json::<sky_tracer::protocol::airports::SearchAirportsResponse>()
                                                .await
                                            {
                                                if let Some(airport) = search_response.airports.first() {
                                                    airport_map.insert(
                                                        airport.code.clone(),
                                                        Airport {
                                                            code: airport.code.clone(),
                                                            position: (
                                                                airport.position.latitude,
                                                                airport.position.longitude,
                                                            ),
                                                        },
                                                    );
                                                }
                                            }
                                        }
                                        Err(e) => warn!("Failed to fetch airport {}: {}", code, e),
                                    }
                                }

                                // Convert flight responses to Flight objects
                                let mut flight_objects: Vec<Flight> = Vec::new();
                                for flight_response in flight_list {
                                    if let (Some(departure), Some(arrival)) = (
                                        airport_map.get(&flight_response.departure),
                                        airport_map.get(&flight_response.arrival),
                                    ) {
                                        flight_objects.push(Flight {
                                            flight_number: flight_response.flight_number.clone(),
                                            departure: departure.clone(),
                                            arrival: arrival.clone(),
                                            position: None,
                                        });
                                    }
                                }

                                info!("Created {} flight objects", flight_objects.len());

                                // Update states
                                let airport_list: Vec<Airport> =
                                    airport_map.into_values().collect();
                                airports.set(airport_list);
                                flights.set(flight_objects);
                            }
                            Err(e) => warn!("Failed to parse flight list: {}", e),
                        }
                    }
                    Err(e) => warn!("Failed to fetch flights: {}", e),
                }
                loading.set(false);
            }

            // Initial fetch
            wasm_bindgen_futures::spawn_local(fetch_data(
                flights.clone(),
                airports.clone(),
                loading.clone(),
            ));

            let flights = flights.clone();
            let airports = airports.clone();
            let loading = loading.clone();

            let interval = gloo_timers::callback::Interval::new(5000, move || {
                wasm_bindgen_futures::spawn_local(fetch_data(
                    flights.clone(),
                    airports.clone(),
                    loading.clone(),
                ));
            });

            move || drop(interval)
        });
    }

    // Fetch positions (only when we have flights)
    {
        let flights = flights.clone();

        use_effect_with((flights.clone(),), move |_| {
            // Depend on flights
            let flights = flights.clone();

            async fn fetch_positions(flights: UseStateHandle<Vec<Flight>>) {
                let mut updated_flights = (*flights).clone();

                // Update positions for all flights
                for flight in &mut updated_flights {
                    if let Ok(response) =
                        Request::get(&format!("/api/flights/{}/position", flight.flight_number))
                            .send()
                            .await
                    {
                        if let Ok(position) = response.json::<FlightPositionResponse>().await {
                            flight.position = Some((position.latitude, position.longitude));
                        }
                    }
                }

                if !updated_flights.is_empty() {
                    flights.set(updated_flights);
                }
            }

            if !flights.is_empty() {
                info!("Fetching positions for {} flights", (*flights).len());
                wasm_bindgen_futures::spawn_local(fetch_positions(flights.clone()));
            }

            || ()
        });
    }

    html! {
        <div class="flight-tracker">
            if *loading {
                <div class="loading">{"Loading flight data..."}</div>
            } else {
                <>
                    <FlightMap
                        flights={(*flights).clone()}
                        airports={(*airports).clone()}
                    />
                    <div class="airport-list">
                        <h3>{"Active Airports"}</h3>
                        <ul>
                            {for airports.iter().map(|airport| html! {
                                <li key={airport.code.clone()}>
                                    {format!("{} ({:.2}°, {:.2}°)",
                                        airport.code,
                                        airport.position.0,
                                        airport.position.1
                                    )}
                                </li>
                            })}
                        </ul>
                    </div>
                </>
            }
        </div>
    }
}
