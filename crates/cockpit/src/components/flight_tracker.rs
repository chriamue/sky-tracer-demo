use super::FlightMap;
use crate::models::{Airport, Flight};
use crate::services::DataService;
use log::info;
use yew::prelude::*;

#[function_component(FlightTracker)]
pub fn flight_tracker() -> Html {
    let flights = use_state(Vec::<Flight>::new);
    let airports = use_state(Vec::<Airport>::new);
    let loading = use_state(|| true);

    // Fetch flights and airports data
    {
        let flights = flights.clone();
        let airports = airports.clone();
        let loading = loading.clone();

        use_effect_with((), move |_| {
            let flights = flights.clone();
            let airports = airports.clone();
            let loading = loading.clone();

            let fetch_data = move || {
                let flights = flights.clone();
                let airports = airports.clone();
                let loading = loading.clone();

                wasm_bindgen_futures::spawn_local(async move {
                    match DataService::get_flights_with_data().await {
                        Ok((flight_data, airport_data)) => {
                            info!(
                                "Loaded {} flights and {} airports",
                                flight_data.len(),
                                airport_data.len()
                            );
                            flights.set(flight_data);
                            airports.set(airport_data);
                        }
                        Err(err) => {
                            log::error!("Failed to load flight data: {}", err);
                        }
                    }
                    loading.set(false);
                });
            };

            // Initial fetch
            fetch_data();

            // Set up periodic updates
            let interval = gloo_timers::callback::Interval::new(5000, move || {
                fetch_data();
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
