use sky_tracer::protocol::satellite::CalculatePositionResponse;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PositionDisplayProps {
    pub data: CalculatePositionResponse,
}

#[function_component(PositionDisplay)]
pub fn position_display(props: &PositionDisplayProps) -> Html {
    html! {
        <div class="position-info">
            <h3>{"Current Position"}</h3>
            {
                if props.data.positions.is_empty() {
                    html! {"No active satellites"}
                } else {
                    html! {
                        <>
                            <p>{"Latitude: "}{props.data.positions[0].latitude}</p>
                            <p>{"Longitude: "}{props.data.positions[0].longitude}</p>
                            <p>{"Altitude: "}{props.data.positions[0].altitude}</p>
                        </>
                    }
                }
            }

            <h3>{"Flight Information"}</h3>
            <div class="flight-info">
                {
                    if let Some(dep) = &props.data.departure_airport {
                        html! {
                            <p>{"Departure: "}{&dep.name}{" ("}{&dep.code}{")"}</p>
                        }
                    } else {
                        html! {}
                    }
                }
                {
                    if let Some(arr) = &props.data.arrival_airport {
                        html! {
                            <p>{"Arrival: "}{&arr.name}{" ("}{&arr.code}{")"}</p>
                        }
                    } else {
                        html! {}
                    }
                }
            </div>
        </div>
    }
}
