use sky_tracer::protocol::satellite::CalculatePositionResponse;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct FlightMapProps {
    pub data: CalculatePositionResponse,
}

#[function_component(FlightMap)]
pub fn flight_map(props: &FlightMapProps) -> Html {
    html! {
        <div class="map-section">
            <h3>{"Flight Map"}</h3>
            <div class="map-container">
                if !props.data.positions.is_empty() {
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
                } else {
                    {"No position data available"}
                }
            </div>
        </div>
    }
}
