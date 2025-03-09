use gloo_net::http::Request;
use sky_tracer::protocol::flights::CreateFlightRequest;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[function_component(FlightForm)]
pub fn flight_form() -> Html {
    let aircraft_ref = use_node_ref();
    let departure_ref = use_node_ref();
    let arrival_ref = use_node_ref();
    let departure_time_ref = use_node_ref();
    let arrival_time_ref = use_node_ref();
    let status = use_state(|| None::<String>);

    // Set default values when component mounts
    {
        let aircraft_ref = aircraft_ref.clone();
        let departure_ref = departure_ref.clone();
        let arrival_ref = arrival_ref.clone();
        let departure_time_ref = departure_time_ref.clone();
        let arrival_time_ref = arrival_time_ref.clone();

        use_effect_with((), move |_| {
            // Set default values
            if let Some(input) = aircraft_ref.cast::<HtmlInputElement>() {
                input.set_value("X-A320");
            }
            if let Some(input) = departure_ref.cast::<HtmlInputElement>() {
                input.set_value("FRA");
            }
            if let Some(input) = arrival_ref.cast::<HtmlInputElement>() {
                input.set_value("LIS");
            }

            // Get current UTC time and add 10 minutes
            let utc_now = chrono::Utc::now();
            let departure_time = utc_now;

            if let Some(input) = departure_time_ref.cast::<HtmlInputElement>() {
                // Format in UTC time
                input.set_value(&departure_time.format("%Y-%m-%dT%H:%M:%SZ").to_string());
            }

            // Set arrival time to 2 hours after departure (in UTC time)
            let arrival_time = departure_time + chrono::Duration::minutes(10);
            if let Some(input) = arrival_time_ref.cast::<HtmlInputElement>() {
                input.set_value(&arrival_time.format("%Y-%m-%dT%H:%M:%SZ").to_string());
            }

            || ()
        });
    }

    let onsubmit = {
        let aircraft_ref = aircraft_ref.clone();
        let departure_ref = departure_ref.clone();
        let arrival_ref = arrival_ref.clone();
        let departure_time_ref = departure_time_ref.clone();
        let arrival_time_ref = arrival_time_ref.clone();
        let status = status.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            let status = status.clone();

            let aircraft = aircraft_ref.cast::<HtmlInputElement>().unwrap().value();
            let departure = departure_ref.cast::<HtmlInputElement>().unwrap().value();
            let arrival = arrival_ref.cast::<HtmlInputElement>().unwrap().value();
            let departure_time = departure_time_ref
                .cast::<HtmlInputElement>()
                .unwrap()
                .value();
            let arrival_time = arrival_time_ref.cast::<HtmlInputElement>().unwrap().value();

            // Use parsed times directly from the input
            let parsed_departure_time = match chrono::DateTime::parse_from_rfc3339(&departure_time)
            {
                Ok(time) => time.with_timezone(&chrono::Utc),
                Err(err) => {
                    status.set(Some(format!("Error parsing departure time: {}", err)));
                    return;
                }
            };

            let parsed_arrival_time = if !arrival_time.is_empty() {
                match chrono::DateTime::parse_from_rfc3339(&arrival_time) {
                    Ok(time) => Some(time.with_timezone(&chrono::Utc)),
                    Err(err) => {
                        status.set(Some(format!("Error parsing arrival time: {}", err)));
                        return;
                    }
                }
            } else {
                None
            };

            let request = CreateFlightRequest {
                aircraft_number: aircraft,
                departure,
                arrival,
                departure_time: parsed_departure_time,
                arrival_time: parsed_arrival_time,
            };

            spawn_local(async move {
                match Request::post("/api/flights")
                    .json(&request)
                    .unwrap()
                    .send()
                    .await
                {
                    Ok(_) => {
                        status.set(Some("Flight created successfully".to_string()));
                    }
                    Err(err) => status.set(Some(format!("Error: {}", err))),
                }
            });
        })
    };

    html! {
        <div class="flight-form">
            <h2>{"Create New Flight"}</h2>
            <form {onsubmit}>
                <div class="form-group">
                    <label for="aircraft">{"Aircraft Number"}</label>
                    <input
                        type="text"
                        id="aircraft"
                        ref={aircraft_ref}
                        required=true
                        placeholder="LH-A320"
                    />
                </div>
                <div class="form-group">
                    <label for="departure">{"Departure Airport"}</label>
                    <input
                        type="text"
                        id="departure"
                        ref={departure_ref}
                        required=true
                        placeholder="FRA"
                    />
                </div>
                <div class="form-group">
                    <label for="arrival">{"Arrival Airport"}</label>
                    <input
                        type="text"
                        id="arrival"
                        ref={arrival_ref}
                        required=true
                        placeholder="LIS"
                    />
                </div>
                <div class="form-group">
                    <label for="departure_time">{"Departure Time"}</label>
                    <input
                        type="datetime"
                        id="departure_time"
                        ref={departure_time_ref}
                        required=true
                        step="60"
                    />
                </div>
                <div class="form-group">
                    <label for="arrival_time">{"Arrival Time"}</label>
                    <input
                        type="datetime"
                        id="arrival_time"
                        ref={arrival_time_ref}
                        step="60"
                    />
                </div>
                <button type="submit">{"Create Flight"}</button>
            </form>
            if let Some(message) = &*status {
                <div class={classes!(
                    "status-message",
                    if message.starts_with("Error") { "error" } else { "success" }
                )}>
                    {message}
                </div>
            }
        </div>
    }
}
