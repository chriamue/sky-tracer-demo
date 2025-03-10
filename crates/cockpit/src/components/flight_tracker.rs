use super::FlightMap;
use gloo_net::http::Request;
use sky_tracer::protocol::airports::AirportResponse;
use sky_tracer::protocol::flights::{FlightPositionResponse, FlightResponse};
use std::collections::HashMap;
use yew::prelude::*;

#[function_component(FlightTracker)]
pub fn flight_tracker() -> Html {
    let flights = use_state(Vec::<FlightResponse>::new);
    let positions = use_state(HashMap::<String, FlightPositionResponse>::new);
    let airports = use_state(HashMap::<String, AirportResponse>::new);
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
                flights: UseStateHandle<Vec<FlightResponse>>,
                airports: UseStateHandle<HashMap<String, AirportResponse>>,
                loading: UseStateHandle<bool>,
            ) {
                // Fetch flights
                if let Ok(response) = Request::get("/api/flights").send().await {
                    if let Ok(flight_list) = response.json::<Vec<FlightResponse>>().await {
                        flights.set(flight_list.clone());

                        // Fetch airports for each flight
                        for flight in flight_list {
                            for code in [&flight.departure, &flight.arrival] {
                                if !airports.contains_key(code) {
                                    if let Ok(response) =
                                        Request::get(&format!("/api/airports/search?code={}", code))
                                            .send()
                                            .await
                                    {
                                        if let Ok(mut airport_list) =
                                            response.json::<Vec<AirportResponse>>().await
                                        {
                                            if let Some(airport) = airport_list.pop() {
                                                airports.set({
                                                    let mut map = (*airports).clone();
                                                    map.insert(code.clone(), airport);
                                                    map
                                                });
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
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

    // Fetch positions
    {
        let flights = flights.clone();
        let positions = positions.clone();

        use_effect_with((flights.clone(),), move |_| {
            let flights = flights.clone();
            let positions = positions.clone();

            async fn fetch_positions(
                flights: UseStateHandle<Vec<FlightResponse>>,
                positions: UseStateHandle<HashMap<String, FlightPositionResponse>>,
            ) {
                for flight in flights.iter() {
                    if let Ok(response) =
                        Request::get(&format!("/api/flights/{}/position", flight.flight_number))
                            .send()
                            .await
                    {
                        if let Ok(position) = response.json::<FlightPositionResponse>().await {
                            positions.set({
                                let mut map = (*positions).clone();
                                map.insert(flight.flight_number.clone(), position);
                                map
                            });
                        }
                    }
                }
            }

            // Initial fetch
            wasm_bindgen_futures::spawn_local(fetch_positions(flights.clone(), positions.clone()));

            let flights = flights.clone();
            let positions = positions.clone();

            let interval = gloo_timers::callback::Interval::new(5000, move || {
                wasm_bindgen_futures::spawn_local(fetch_positions(
                    flights.clone(),
                    positions.clone(),
                ));
            });

            move || drop(interval)
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
                        positions={(*positions).clone()}
                        airports={(*airports).clone()}
                    />
                    <div class="airport-list">
                        <h3>{"Active Airports"}</h3>
                        <ul>
                            {for airports.values().map(|airport| html! {
                                <li key={airport.code.clone()}>
                                    {format!("{} ({:.2}°, {:.2}°)",
                                        airport.code,
                                        airport.position.latitude,
                                        airport.position.longitude
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
