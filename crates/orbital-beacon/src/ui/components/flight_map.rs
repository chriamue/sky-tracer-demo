use serde_json::json;
use sky_tracer::protocol::satellite::CalculatePositionResponse;
use std::f64::consts::PI;
use uuid::Uuid;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct FlightMapProps {
    pub data: CalculatePositionResponse,
}

fn calculate_bearing(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let d_lon = (lon2 - lon1) * PI / 180.0;
    let lat1_rad = lat1 * PI / 180.0;
    let lat2_rad = lat2 * PI / 180.0;

    let y = d_lon.sin() * lat2_rad.cos();
    let x = lat1_rad.cos() * lat2_rad.sin() - lat1_rad.sin() * lat2_rad.cos() * d_lon.cos();

    let mut bearing = y.atan2(x) * 180.0 / PI;
    if bearing < 0.0 {
        bearing += 360.0;
    }
    bearing
}

#[function_component(FlightMap)]
pub fn flight_map(props: &FlightMapProps) -> Html {
    let map_id = format!("map-{}", Uuid::new_v4());
    let has_data = !props.data.positions.is_empty();

    let map_data = if has_data {
        let current_pos = &props.data.positions[0];
        let dep_pos = props
            .data
            .departure_airport
            .as_ref()
            .map(|a| (a.position.latitude, a.position.longitude));
        let arr_pos = props
            .data
            .arrival_airport
            .as_ref()
            .map(|a| (a.position.latitude, a.position.longitude));

        // Calculate bearing if we have both departure and arrival
        let bearing = match (dep_pos, arr_pos) {
            (Some((dep_lat, dep_lon)), Some((arr_lat, arr_lon))) => {
                calculate_bearing(dep_lat, dep_lon, arr_lat, arr_lon)
            }
            _ => 0.0,
        };

        let map_data = json!({
            "currentPosition": [
                current_pos.latitude,
                current_pos.longitude
            ],
            "departure": [
                props.data.departure_airport.as_ref().map(|a| a.position.latitude).unwrap_or(0.0),
                props.data.departure_airport.as_ref().map(|a| a.position.longitude).unwrap_or(0.0)
            ],
            "arrival": [
                props.data.arrival_airport.as_ref().map(|a| a.position.latitude).unwrap_or(0.0),
                props.data.arrival_airport.as_ref().map(|a| a.position.longitude).unwrap_or(0.0)
            ],
            "depCode": props.data.departure_airport.as_ref().map_or(String::new(), |a| a.code.clone()),
            "arrCode": props.data.arrival_airport.as_ref().map_or(String::new(), |a| a.code.clone()),
            "bearing": bearing
        });
        map_data.to_string()
    } else {
        String::new()
    };

    html! {
        <div class="map-section">
            <h3>{"Flight Map"}</h3>
            <div class="map-container">
                if has_data {
                    <>
                        <div id={map_id.clone()} data-map={map_data} style="width: 100%; height: 400px;"></div>
                        <div class="map-info">
                            <p>{"Current Position:"}</p>
                            <p>{format!("({:.6}, {:.6})",
                                props.data.positions[0].latitude,
                                props.data.positions[0].longitude)}
                            </p>
                            if let (Some(dep), Some(arr)) = (&props.data.departure_airport, &props.data.arrival_airport) {
                                <div class="map-route">
                                    <p>{"Route: "}{&dep.code}{" → "}{&arr.code}</p>
                                </div>
                            }
                        </div>
                    </>
                } else {
                    {"No position data available"}
                }
            </div>
        </div>
    }
}
