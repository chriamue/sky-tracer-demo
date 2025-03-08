use crate::ui::components::PositionForm;
use sky_tracer::protocol::satellite::CalculatePositionResponse;
use yew::prelude::*;

#[derive(Default, Properties, PartialEq)]
pub struct FlightPositionProps {
    pub position_data: Option<CalculatePositionResponse>,
}

#[function_component(FlightPosition)]
pub fn flight_position(props: &FlightPositionProps) -> Html {
    html! {
        <div class="container">
            <header>
                <h1>{"Flight Position"}</h1>
                <a href={format!("{}/", crate::utils::get_path_prefix())} class="back-link">{"Back to Home"}</a>
            </header>

            <main>
                <PositionForm />

                {
                    match &props.position_data {
                        Some(data) => {
                            html! {
                                <div class="position-info">
                                    <h3>{"Current Position"}</h3>
                                    {
                                        if data.positions.is_empty() {
                                            html! {"No active satellites"}
                                        } else {
                                            html! {
                                                <>
                                                    <p>{"Latitude: "}{data.positions[0].latitude}</p>
                                                    <p>{"Longitude: "}{data.positions[0].longitude}</p>
                                                    <p>{"Altitude: "}{data.positions[0].altitude}</p>
                                                </>
                                            }
                                        }
                                    }

                                    <h3>{"Flight Information"}</h3>
                                    <div class="flight-info">
                                        {
                                            if let Some(dep) = &data.departure_airport {
                                                html! {
                                                    <p>{"Departure: "}{&dep.name}{" ("}{&dep.code}{")"}</p>
                                                }
                                            } else {
                                                html! {}
                                            }
                                        }
                                        {
                                            if let Some(arr) = &data.arrival_airport {
                                                html! {
                                                    <p>{"Arrival: "}{&arr.name}{" ("}{&arr.code}{")"}</p>
                                                }
                                            } else {
                                                html! {}
                                            }
                                        }
                                    </div>

                                    <h3>{"Map"}</h3>
                                    <div class="map-container">
                                        {"[Map Placeholder]"}
                                    </div>
                                </div>
                            }
                        }
                        None => {
                            html! {
                                <div class="instructions">
                                    <p>{"Enter flight details to calculate position"}</p>
                                </div>
                            }
                        }
                    }
                }
            </main>
        </div>
    }
}
