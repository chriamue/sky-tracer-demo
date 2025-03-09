use gloo_net::http::Request;
use sky_tracer::protocol::flights::FlightResponse;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[derive(Debug, PartialEq)]
enum ConnectionStatus {
    Connected,
    Disconnected,
    Checking,
}

#[function_component(StatusPanel)]
pub fn status_panel() -> Html {
    let connection_status = use_state(|| ConnectionStatus::Checking);
    let flight_count = use_state(|| 0);
    let last_update = use_state(|| chrono::Local::now());

    // Check API connection and fetch flight count periodically
    {
        let connection_status = connection_status.clone();
        let flight_count = flight_count.clone();
        let last_update = last_update.clone();

        use_effect_with((), move |_| {
            let update = move || {
                let connection_status = connection_status.clone();
                let flight_count = flight_count.clone();
                let last_update = last_update.clone();

                spawn_local(async move {
                    match Request::get("/api/flights").send().await {
                        Ok(response) => {
                            if response.ok() {
                                connection_status.set(ConnectionStatus::Connected);
                                if let Ok(flights) = response.json::<Vec<FlightResponse>>().await {
                                    flight_count.set(flights.len());
                                }
                            } else {
                                connection_status.set(ConnectionStatus::Disconnected);
                            }
                        }
                        Err(_) => {
                            connection_status.set(ConnectionStatus::Disconnected);
                        }
                    }
                    last_update.set(chrono::Local::now());
                });
            };

            // Initial update
            update();

            // Set up interval for periodic updates
            let interval = gloo_timers::callback::Interval::new(5_000, move || {
                update();
            });

            move || drop(interval)
        });
    }

    let connection_class = match *connection_status {
        ConnectionStatus::Connected => "status-ok",
        ConnectionStatus::Disconnected => "status-error",
        ConnectionStatus::Checking => "",
    };

    let connection_text = match *connection_status {
        ConnectionStatus::Connected => "Connected",
        ConnectionStatus::Disconnected => "Disconnected",
        ConnectionStatus::Checking => "Checking...",
    };

    // Calculate system load based on number of flights (example logic)
    let system_load = (*flight_count as f32 * 6.25).min(100.0); // Each flight adds 6.25% load
    let load_class = if system_load > 80.0 {
        "status-error"
    } else if system_load > 60.0 {
        "status-warning"
    } else {
        "status-ok"
    };

    html! {
        <div class="status-panel">
            <h2>{"System Status"}</h2>
            <div class="status-grid">
                <div class="status-item">
                    <span class="status-label">{"API Connection"}</span>
                    <span class={classes!("status-value", connection_class)}>
                        {connection_text}
                    </span>
                </div>
                <div class="status-item">
                    <span class="status-label">{"Active Flights"}</span>
                    <span class="status-value">{*flight_count}</span>
                </div>
                <div class="status-item">
                    <span class="status-label">{"System Load"}</span>
                    <span class={classes!("status-value", load_class)}>
                        {format!("{:.0}%", system_load)}
                    </span>
                </div>
                <div class="status-item">
                    <span class="status-label">{"Last Update"}</span>
                    <span class="status-value">
                        {last_update.format("%H:%M:%S").to_string()}
                    </span>
                </div>
            </div>
        </div>
    }
}
