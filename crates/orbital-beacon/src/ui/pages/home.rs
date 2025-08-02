use crate::ui::components::SatelliteList;
use sky_tracer::protocol::satellite::SatelliteResponse;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct HomeProps {
    pub satellites: Vec<SatelliteResponse>,
}

#[function_component(Home)]
pub fn home(props: &HomeProps) -> Html {
    let path_prefix = crate::utils::get_path_prefix();

    html! {
        <div class="container">
            <header>
                <h1>{"🛰️ Orbital Beacon"}</h1>
                <p class="subtitle">{"Satellite Control & Flight Tracking"}</p>
                <div class="header-links">
                    <a href={format!("{}/launch", path_prefix)} class="launch-link">
                        {"Launch New Satellite"}
                    </a>
                    <a href={format!("{}/flight_position", path_prefix)} class="launch-link">
                        {"Track Flight Position"}
                    </a>
                </div>
            </header>

            <main>
                <div class="grid">
                    <div class="panel">
                        <SatelliteList satellites={props.satellites.clone()} />
                    </div>
                </div>
            </main>
        </div>
    }
}
