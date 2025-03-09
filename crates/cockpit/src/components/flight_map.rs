use gloo_net::http::Request;
use serde_json::json;
use sky_tracer::protocol::flights::FlightResponse;
use uuid::Uuid;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::window;
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window)]
    fn initializeFlightsMap(containerId: &str, mapData: &str);
}

#[function_component(FlightMap)]
pub fn flight_map() -> Html {
    let map_id = use_state(|| format!("map-{}", Uuid::new_v4()));
    let flights = use_state(Vec::<FlightResponse>::new);
    let loading = use_state(|| true);

    // Function to fetch flights data
    let fetch_flights = {
        let flights = flights.clone();
        let loading = loading.clone();
        let map_id = map_id.clone();

        move || {
            let flights = flights.clone();
            let loading = loading.clone();
            let map_id = map_id.clone();

            spawn_local(async move {
                match Request::get("/api/flights").send().await {
                    Ok(response) => {
                        if let Ok(data) = response.json::<Vec<FlightResponse>>().await {
                            flights.set(data);

                            // Initialize/Update map after data is loaded
                            let map_data = json!({
                                "flights": (*flights).iter().map(|flight| json!({
                                    "flightNumber": flight.flight_number,
                                    "departure": {
                                        "code": flight.departure,
                                        "position": [50.033333, 8.570556]
                                    },
                                    "arrival": {
                                        "code": flight.arrival,
                                        "position": [38.7223, -9.1393]
                                    },
                                    "position": null
                                })).collect::<Vec<_>>()
                            });

                            // Call the JavaScript function directly
                            initializeFlightsMap(&map_id, &map_data.to_string());
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

        use_effect_with((), move |_| {
            // Initial fetch
            fetch_flights();

            // Set up interval for periodic updates
            let interval = gloo_timers::callback::Interval::new(10_000, move || {
                fetch_flights();
            });

            // Cleanup function to remove interval when component unmounts
            move || drop(interval)
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
