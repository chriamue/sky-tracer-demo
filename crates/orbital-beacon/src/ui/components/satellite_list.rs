use sky_tracer::protocol::protocol::SatelliteResponse;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SatelliteListProps {
    pub satellites: Vec<SatelliteResponse>,
}

#[function_component(SatelliteList)]
pub fn satellite_list(props: &SatelliteListProps) -> Html {
    html! {
        <div class="satellite-list">
            <h2>{"Satellites"}</h2>
            <div class="satellite-grid">
                {for props.satellites.iter().map(|satellite| {
                    let status_class = match satellite.status {
                        sky_tracer::model::SatelliteStatus::Active => "status-active",
                        sky_tracer::model::SatelliteStatus::Inactive => "status-inactive",
                        sky_tracer::model::SatelliteStatus::Maintenance => "status-maintenance",
                    };
                    html! {
                        <div class="satellite-card">
                            <h3>{&satellite.name}</h3>
                            <p class={classes!("status", status_class)}>
                                {format!("Status: {:?}", satellite.status)}
                            </p>
                            <p class="id">{"ID: "}{satellite.id}</p>
                            <form
                                action={format!("/update_status/{}", satellite.id)}
                                method="POST"
                                class="satellite-form"
                            >
                                <select name="status">
                                    <option value="Active">{"Active"}</option>
                                    <option value="Inactive">{"Inactive"}</option>
                                    <option value="Maintenance">{"Maintenance"}</option>
                                </select>
                                <button type="submit">{"Update Status"}</button>
                            </form>
                        </div>
                    }
                })}
            </div>
        </div>
    }
}
