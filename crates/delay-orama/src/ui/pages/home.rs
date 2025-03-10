use crate::ui::components::DelayTable;
use sky_tracer::protocol::flights::{FlightPositionResponse, FlightResponse};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct HomeProps {
    pub flights: Vec<(FlightResponse, Option<FlightPositionResponse>)>,
    pub airport_position: Option<(f64, f64)>,
}

#[function_component(Home)]
pub fn home(props: &HomeProps) -> Html {
    html! {
        <div class="container">
            <header>
                <h1>{"⏰ Delay-O-Rama"}</h1>
                <p>{"Real-time Flight Delay Information"}</p>
            </header>

            <main>
                <DelayTable
                    flights={props.flights.clone()}
                    airport_position={props.airport_position}
                />
            </main>
        </div>
    }
}
