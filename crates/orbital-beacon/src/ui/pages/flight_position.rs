use crate::ui::components::{FlightMap, PositionDisplay, PositionForm};
use sky_tracer::protocol::satellite::CalculatePositionResponse;
use yew::prelude::*;

#[derive(Default, Properties, PartialEq)]
pub struct FlightPositionProps {
    pub position_data: Option<CalculatePositionResponse>,
    pub error_message: Option<String>,
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
                    if let Some(error) = &props.error_message {
                        html! {
                            <div class="error-message">
                                <p>{error}</p>
                            </div>
                        }
                    } else {
                        match &props.position_data {
                            Some(data) => {
                                html! {
                                    <div class="position-container">
                                        <PositionDisplay data={data.clone()} />
                                        <FlightMap data={data.clone()} />
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
                }
            </main>
        </div>
    }
}
