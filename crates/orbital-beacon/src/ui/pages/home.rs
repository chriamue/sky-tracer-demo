use crate::ui::components::{PositionForm, SatelliteList};
use sky_tracer::protocol::satellite::SatelliteResponse;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct HomeProps {
    pub satellites: Vec<SatelliteResponse>,
}

#[function_component(Home)]
pub fn home(props: &HomeProps) -> Html {
    html! {
        <div class="container">
            <header>
                <h1>{"🛰️ Orbital Beacon"}</h1>
                <p class="subtitle">{"Satellite Control & Flight Tracking"}</p>
                <a href="/launch" class="launch-link">{"Launch New Satellite"}</a>
            </header>

            <main>
                <div class="grid">
                    <div class="left-panel">
                        <SatelliteList satellites={props.satellites.clone()} />
                    </div>
                    <div class="right-panel">
                        <PositionForm />
                    </div>
                </div>
            </main>
        </div>
    }
}
