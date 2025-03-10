use serde_json::json;
use sky_tracer::protocol::airports::AirportResponse;
use sky_tracer::protocol::flights::{FlightPositionResponse, FlightResponse};
use std::collections::HashMap;
use uuid::Uuid;
use web_sys::wasm_bindgen::JsCast;
use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct FlightMapProps {
    pub flights: Vec<FlightResponse>,
    pub positions: HashMap<String, FlightPositionResponse>,
    pub airports: HashMap<String, AirportResponse>,
}

#[function_component(FlightMap)]
pub fn flight_map(props: &FlightMapProps) -> Html {
    let map_id = use_state(|| format!("map-{}", Uuid::new_v4()));

    // Function to update map
    let update_map = {
        let map_id = map_id.clone();
        let props = props.clone();

        move || {
            let map_data = json!({
                "flights": props.flights.iter().map(|flight| {
                    let departure_pos = props.airports.get(&flight.departure)
                        .map(|a| [a.position.latitude, a.position.longitude]);
                    let arrival_pos = props.airports.get(&flight.arrival)
                        .map(|a| [a.position.latitude, a.position.longitude]);
                    let position = props.positions.get(&flight.flight_number)
                        .map(|p| [p.latitude, p.longitude]);

                    json!({
                        "flightNumber": flight.flight_number,
                        "departure": {
                            "code": flight.departure,
                            "position": departure_pos.unwrap_or([50.033333, 8.570556])
                        },
                        "arrival": {
                            "code": flight.arrival,
                            "position": arrival_pos.unwrap_or([38.7223, -9.1393])
                        },
                        "position": position
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

    // Update map when props change
    use_effect_with(
        (
            props.flights.clone(),
            props.positions.clone(),
            props.airports.clone(),
        ),
        move |_| {
            update_map();
            || ()
        },
    );

    html! {
        <div class="map-section">
            <h2>{"Live Flight Map"}</h2>
            <div id={(*map_id).clone()} class="map-container"></div>
        </div>
    }
}
