use chrono::{DateTime, Duration, Utc};
use yew::prelude::*;

fn get_path_prefix() -> String {
    std::env::var("PATH_PREFIX").unwrap_or_else(|_| "".to_string())
}

fn calculate_default_times(current_time: DateTime<Utc>) -> (DateTime<Utc>, DateTime<Utc>) {
    let departure_time = current_time - Duration::minutes(1);
    let arrival_time = current_time + Duration::minutes(4);
    (departure_time, arrival_time)
}

#[function_component(PositionForm)]
pub fn position_form() -> Html {
    let now = Utc::now();
    let (departure_time, arrival_time) = calculate_default_times(now);

    html! {
        <div class="position-form">
            <h2>{"Calculate Flight Position"}</h2>
            <form
                action={format!("{}/flight_position", get_path_prefix())}
                method="GET"
                class="position-form"
            >
                <div class="form-group">
                    <label for="flight_number">{"Flight Number:"}</label>
                    <input
                        type="text"
                        id="flight_number"
                        name="flight_number"
                        required=true
                        placeholder="Enter flight number"
                        value="FRA123"
                    />
                </div>
                <div class="form-group">
                    <label for="departure">{"Departure Airport:"}</label>
                    <input
                        type="text"
                        id="departure"
                        name="departure"
                        required=true
                        placeholder="Enter departure airport"
                        value="FRA"
                    />
                </div>
                <div class="form-group">
                    <label for="arrival">{"Arrival Airport:"}</label>
                    <input
                        type="text"
                        id="arrival"
                        name="arrival"
                        required=true
                        placeholder="Enter arrival airport"
                        value="LIS"
                    />
                </div>
                <div class="form-group">
                    <label for="departure_time">{"Departure Time:"}</label>
                    <input
                        type="datetime-local"
                        id="departure_time"
                        name="departure_time"
                        required=true
                        value={departure_time.format("%Y-%m-%dT%H:%M").to_string()}
                    />
                </div>
                <div class="form-group">
                    <label for="arrival_time">{"Arrival Time:"}</label>
                    <input
                        type="datetime-local"
                        id="arrival_time"
                        name="arrival_time"
                        required=true
                        value={arrival_time.format("%Y-%m-%dT%H:%M").to_string()}
                    />
                </div>
                <button type="submit">{"Calculate Position"}</button>
            </form>
        </div>
    }
}
