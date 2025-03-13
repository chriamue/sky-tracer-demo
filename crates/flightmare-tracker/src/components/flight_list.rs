use crate::components::FlightWithDelayComponent;
use crate::grund::get_random_grund;
use crate::FlightWithDelay;
use gloo_net::http::Request;
use gloo_timers::future::sleep;
use sky_tracer::protocol::flights::FlightResponse;
use std::time::Duration;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[function_component(FlightList)]
pub fn flight_list() -> Html {
    let flights = use_state(Vec::<FlightWithDelay>::new);
    let loading = use_state(|| true);
    let error = use_state(|| Option::<String>::None);

    {
        let flights = flights.clone();
        let loading = loading.clone();
        let error = error.clone();

        use_effect_with(flights.clone(), move |_| {
            spawn_local(async move {
                sleep(Duration::from_secs(2)).await;
                loading.set(true);
                sleep(Duration::from_secs(2)).await;
                match Request::get("/api/flights").send().await {
                    Ok(response) => match response.json::<Vec<FlightResponse>>().await {
                        Ok(flight_data) => {
                            let flights_with_delay: Vec<FlightWithDelay> = flight_data
                                .into_iter()
                                .map(|flight| {
                                    if rand::random::<f32>() < 0.5 {
                                        FlightWithDelay::with_grund(flight, get_random_grund())
                                    } else {
                                        flight.into()
                                    }
                                })
                                .collect();
                            flights.set(flights_with_delay);
                            error.set(None);
                        }
                        Err(e) => error.set(Some(format!("Failed to parse flight data: {}", e))),
                    },
                    Err(e) => error.set(Some(format!("Failed to fetch flights: {}", e))),
                }
                loading.set(false);
            });
            || ()
        });
    }

    html! {
        <div class="flight-list">
            if *loading {
                <div class="loading">
                    <div class="loading-sequence">
                        <span>{"✈️"}</span>
                        <span>{"💨"}</span>
                        <span>{"🌥️"}</span>
                    </div>
                    <p>{"Finding excuses..."}</p>
                </div>
            } else if let Some(err) = &*error {
                <div class="error-message">
                    <p>{err}</p>
                </div>
            } else {
                {flights.iter().map(|flight_with_delay| {
                    html! {
                        <FlightWithDelayComponent flight_with_delay={flight_with_delay.clone()} />
                    }
                }).collect::<Html>()}
            }
        </div>
    }
}
