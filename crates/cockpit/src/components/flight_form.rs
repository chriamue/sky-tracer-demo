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
    let time_ref = use_node_ref();
    let status = use_state(|| None::<String>);

    // Set default values when component mounts
    {
        let aircraft_ref = aircraft_ref.clone();
        let departure_ref = departure_ref.clone();
        let arrival_ref = arrival_ref.clone();
        let time_ref = time_ref.clone();

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
            if let Some(input) = time_ref.cast::<HtmlInputElement>() {
                // Get current time and format it for datetime-local input
                let now = chrono::Local::now();
                let formatted_time = now.format("%Y-%m-%dT%H:%M").to_string();
                input.set_value(&formatted_time);
            }
            || ()
        });
    }

    let onsubmit = {
        let aircraft_ref = aircraft_ref.clone();
        let departure_ref = departure_ref.clone();
        let arrival_ref = arrival_ref.clone();
        let time_ref = time_ref.clone();
        let status = status.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            let status = status.clone();

            let aircraft = aircraft_ref.cast::<HtmlInputElement>().unwrap().value();
            let departure = departure_ref.cast::<HtmlInputElement>().unwrap().value();
            let arrival = arrival_ref.cast::<HtmlInputElement>().unwrap().value();
            let time = time_ref.cast::<HtmlInputElement>().unwrap().value();

            // Convert datetime-local format to RFC3339
            let parsed_time = match convert_datetime_local_to_utc(&time) {
                Ok(time) => time,
                Err(err) => {
                    status.set(Some(format!("Error parsing time: {}", err)));
                    return;
                }
            };

            let request = CreateFlightRequest {
                aircraft_number: aircraft,
                departure,
                arrival,
                departure_time: parsed_time,
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
                    <label for="time">{"Departure Time"}</label>
                    <input
                        type="datetime-local"
                        id="time"
                        ref={time_ref}
                        required=true
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

fn convert_datetime_local_to_utc(
    datetime_local: &str,
) -> Result<chrono::DateTime<chrono::Utc>, String> {
    // datetime-local format is: "YYYY-MM-DDThh:mm"
    // We need to parse it and convert to UTC
    let datetime = chrono::NaiveDateTime::parse_from_str(
        &format!("{}:00", datetime_local), // Add seconds
        "%Y-%m-%dT%H:%M:%S",
    )
    .map_err(|e| format!("Failed to parse datetime: {}", e))?;

    // Convert to UTC
    Ok(chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(
        datetime,
        chrono::Utc,
    ))
}
