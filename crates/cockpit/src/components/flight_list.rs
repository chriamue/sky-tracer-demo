use gloo_net::http::Request;
use sky_tracer::protocol::flights::FlightResponse;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

fn calculate_flight_progress(flight: &FlightResponse) -> f64 {
    let now = chrono::Utc::now();
    let departure = flight.departure_time;
    let arrival = flight
        .arrival_time
        .unwrap_or_else(|| flight.departure_time + chrono::Duration::hours(2));

    if now < departure {
        0.0
    } else if now > arrival {
        100.0
    } else {
        let total_duration = arrival - departure;
        let elapsed = now - departure;

        (elapsed.num_seconds() as f64 / total_duration.num_seconds() as f64 * 100.0)
            .min(100.0)
            .max(0.0)
    }
}

fn get_progress_class(progress: f64) -> &'static str {
    match progress {
        p if p == 0.0 => "not-started",
        p if p == 100.0 => "completed",
        _ => "in-progress",
    }
}

#[function_component(FlightList)]
pub fn flight_list() -> Html {
    let flights = use_state(Vec::<FlightResponse>::new);
    let loading = use_state(|| true);

    // Function to fetch flights data
    let fetch_flights = {
        let flights = flights.clone();
        let loading = loading.clone();

        move || {
            let flights = flights.clone();
            let loading = loading.clone();

            spawn_local(async move {
                match Request::get("/api/flights").send().await {
                    Ok(response) => {
                        if let Ok(data) = response.json::<Vec<FlightResponse>>().await {
                            flights.set(data);
                        }
                    }
                    Err(err) => log::error!("Error fetching flights: {}", err),
                }
                loading.set(false);
            });
        }
    };

    // Initial load and setup periodic refresh
    {
        let fetch_flights = fetch_flights.clone();

        use_effect_with((), move |_| {
            // Initial fetch
            fetch_flights();

            // Set up interval for periodic updates
            let interval = gloo_timers::callback::Interval::new(5_000, move || {
                fetch_flights();
            });

            // Cleanup function to remove interval when component unmounts
            move || drop(interval)
        });
    }

    html! {
        <div class="flight-list">
            <h2>{"Current Flights"}</h2>
            if *loading {
                <div class="loading">{"Loading..."}</div>
            } else {
                <table>
                    <thead>
                        <tr>
                            <th>{"Flight #"}</th>
                            <th>{"Aircraft"}</th>
                            <th>{"From"}</th>
                            <th>{"To"}</th>
                            <th>{"Departure"}</th>
                            <th>{"Progress"}</th>
                        </tr>
                    </thead>
                    <tbody>
                        {for flights.iter().map(|flight| {
                            let progress = calculate_flight_progress(flight);
                            let progress_class = get_progress_class(progress);

                            html! {
                                <tr>
                                    <td>{&flight.flight_number}</td>
                                    <td>{&flight.aircraft_number}</td>
                                    <td>{&flight.departure}</td>
                                    <td>{&flight.arrival}</td>
                                    <td>{flight.departure_time.format("%Y-%m-%d %H:%M").to_string()}</td>
                                    <td class="progress-cell">
                                        <div class="progress-bar-container">
                                            <div
                                                class={classes!("progress-bar", progress_class)}
                                                style={format!("width: {}%", progress)}
                                            >
                                                {format!("{:.0}%", progress)}
                                            </div>
                                        </div>
                                    </td>
                                </tr>
                            }
                        })}
                    </tbody>
                </table>
            }
        </div>
    }
}
