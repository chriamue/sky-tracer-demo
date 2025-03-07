use yew::prelude::*;

#[function_component(StatusPanel)]
pub fn status_panel() -> Html {
    html! {
        <div class="status-panel">
            <h2>{"System Status"}</h2>
            <div class="status-grid">
                <div class="status-item">
                    <span class="status-label">{"API Connection"}</span>
                    <span class="status-value status-ok">{"Connected"}</span>
                </div>
                <div class="status-item">
                    <span class="status-label">{"Active Flights"}</span>
                    <span class="status-value">{"12"}</span>
                </div>
                <div class="status-item">
                    <span class="status-label">{"System Load"}</span>
                    <span class="status-value status-warning">{"75%"}</span>
                </div>
                <div class="status-item">
                    <span class="status-label">{"Last Update"}</span>
                    <span class="status-value">{chrono::Local::now().format("%H:%M:%S").to_string()}</span>
                </div>
            </div>
        </div>
    }
}
