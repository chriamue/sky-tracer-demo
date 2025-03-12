use sky_tracer::protocol::satellite::SatelliteResponse;
use yew::prelude::*;

fn get_path_prefix() -> String {
    std::env::var("PATH_PREFIX").unwrap_or_else(|_| "".to_string())
}

#[derive(Properties, PartialEq)]
pub struct SatelliteListProps {
    pub satellites: Vec<SatelliteResponse>,
}

#[function_component(SatelliteList)]
pub fn satellite_list(props: &SatelliteListProps) -> Html {
    html! {
        <div class="satellite-list">
            <h2>{"Satellite Control Center"}</h2>
            <div class="orbital-view">
                <div class="earth"></div>
                <div class="orbital-ring"></div>
                {for props.satellites.iter().enumerate().map(|(index, satellite)| {
                    let status_class = match satellite.status {
                        sky_tracer::model::SatelliteStatus::Active => "status-active",
                        sky_tracer::model::SatelliteStatus::Inactive => "status-inactive",
                        sky_tracer::model::SatelliteStatus::Maintenance => "status-maintenance",
                    };
                    let orbit_position = format!("orbit-position-{}", index);

                    html! {
                        <div class={classes!("satellite-container", orbit_position)}>
                            <div class={classes!("satellite", status_class)}> // Added status_class here
                                <div class="satellite-ui-wrapper">
                                    <div class="satellite-name-tag">
                                        {&satellite.name}
                                    </div>
                                    <div class="control-panel">
                                        <form action={format!("{}/update_status/{}", get_path_prefix(), satellite.id)} method="POST">
                                            <select name="status" class="status-select">
                                                <option value="Active">{"Active"}</option>
                                                <option value="Inactive">{"Inactive"}</option>
                                                <option value="Maintenance">{"Maintenance"}</option>
                                            </select>
                                            <button type="submit" class="control-button">{"Update"}</button>
                                        </form>
                                    </div>
                                    <div class="satellite-status-tag">
                                        {format!("Status: {:?}", satellite.status)}
                                    </div>
                                </div>
                                <div class="satellite-body">
                                    <div class="satellite-panel left"></div>
                                    <div class="satellite-core">{"🛰️"}</div>
                                    <div class="satellite-panel right"></div>
                                </div>
                            </div>
                        </div>
                    }
                })}
            </div>
        </div>
    }
}
