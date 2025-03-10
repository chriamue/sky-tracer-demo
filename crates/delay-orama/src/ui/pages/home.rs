use crate::ui::components::DelayTable;
use sky_tracer::protocol::flights::FlightResponse;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct HomeProps {
    pub flights: Vec<FlightResponse>,
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
                <DelayTable flights={props.flights.clone()} />
            </main>
        </div>
    }
}
