use crate::models::{Airport, Flight};
use serde_json::json;
use uuid::Uuid;
use web_sys::wasm_bindgen::JsCast;
use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct FlightMapProps {
    pub flights: Vec<Flight>,
    pub airports: Vec<Airport>,
}

#[function_component(FlightMap)]
pub fn flight_map(props: &FlightMapProps) -> Html {
    let map_id = use_state(|| format!("map-{}", Uuid::new_v4()));

    let update_map = {
        let map_id = map_id.clone();
        let props = props.clone();

        move || {
            let map_data = json!({
                "flights": props.flights.iter().map(|flight| {
                    json!({
                        "flightNumber": flight.flight_number,
                        "departure": {
                            "code": flight.departure.code,
                            "position": [flight.departure.position.0, flight.departure.position.1]
                        },
                        "arrival": {
                            "code": flight.arrival.code,
                            "position": [flight.arrival.position.0, flight.arrival.position.1]
                        },
                        "position": flight.position.map(|p| [p.0, p.1])
                    })
                }).collect::<Vec<_>>()
            });

            if let Some(window) = web_sys::window() {
                if let Some(init_fn) =
                    js_sys::Reflect::get(&window, &"initializeFlightsMap".into()).ok()
                {
                    let _ = init_fn.dyn_into::<js_sys::Function>().unwrap().call2(
                        &window,
                        &(*map_id).as_str().into(),
                        &map_data.to_string().into(),
                    );
                }
            }
        }
    };

    use_effect_with((props.flights.clone(), props.airports.clone()), move |_| {
        update_map();
        || ()
    });

    html! {
        <div class="map-section">
            <h2>{"Live Flight Map"}</h2>
            <div id={(*map_id).clone()} class="map-container"></div>
        </div>
    }
}
