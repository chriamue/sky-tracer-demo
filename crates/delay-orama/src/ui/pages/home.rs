use crate::ui::components::{DelayTable, ErrorMessage};
use sky_tracer::protocol::flights::{FlightPositionResponse, FlightResponse};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct HomeProps {
    pub flights: Vec<(FlightResponse, Option<FlightPositionResponse>)>,
    pub airport_position: Option<(f64, f64)>,
    pub airport_code: Option<String>,
    pub error_message: Option<String>,
}

#[function_component(Home)]
pub fn home(props: &HomeProps) -> Html {
    html! {
        <div class="container">
            <header>
                <h1>{"⏰ Delay-O-Rama"}</h1>
                {
                    if let Some(airport_code) = &props.airport_code {
                        html! {
                            <p>{format!("Real-time Flight Delay Information for {}", airport_code)}</p>
                        }
                    } else {
                        html! {
                            <p>{"Real-time Flight Delay Information"}</p>
                        }
                    }
                }
            </header>

            <main>
                {
                    if let Some(error) = &props.error_message {
                        html! {
                            <ErrorMessage message={error.clone()} />
                        }
                    } else if props.flights.is_empty() && props.airport_code.is_some() {
                        html! {
                            <ErrorMessage message={"No flights found for this airport. Please check the airport code and try again."} />
                        }
                    } else {
                        html! {
                            <DelayTable
                                flights={props.flights.clone()}
                                airport_position={props.airport_position}
                            />
                        }
                    }
                }
            </main>
        </div>
    }
}
