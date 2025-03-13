use crate::components::FlightWithDelayComponent;
use crate::grund::get_random_grund;
use crate::FlightWithDelay;
use gloo_net::http::Request;
use gloo_timers::future::sleep;
use sky_tracer::protocol::flights::FlightResponse;
use std::time::Duration;
use yew::prelude::*;
use yew::suspense::use_future;

async fn fetch_flights() -> Result<Vec<FlightWithDelay>, String> {
    sleep(Duration::from_secs(5)).await;

    let response = Request::get("/api/flights")
        .send()
        .await
        .map_err(|e| format!("Failed to fetch flights: {}", e))?;

    let flights: Vec<FlightResponse> = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse flight data: {}", e))?;

    let flights_with_delay: Vec<FlightWithDelay> = flights
        .into_iter()
        .map(|flight| {
            if rand::random::<f32>() < 0.5 {
                FlightWithDelay::with_grund(flight, get_random_grund())
            } else {
                flight.into()
            }
        })
        .collect();

    Ok(flights_with_delay)
}

#[function_component(FlightListContent)]
fn flight_list_content() -> HtmlResult {
    let flights = use_future(|| async move { fetch_flights().await })?;

    match &*flights {
        Ok(flights) => Ok(html! {
            <div class="flight-list">
                {for flights.iter().map(|flight_with_delay| html! {
                    <FlightWithDelayComponent flight_with_delay={flight_with_delay.clone()} />
                })}
            </div>
        }),
        Err(error) => Ok(html! {
            <div class="error-message">
                <p>{error}</p>
            </div>
        }),
    }
}

#[function_component(AsyncFlightList)]
pub fn async_flight_list() -> Html {
    html! {
        <Suspense fallback={html! {
            <div class="loading">
                <div class="spinner">{"🌪️"}</div>
                <p>{"Finding excuses..."}</p>
            </div>
        }}>
            <FlightListContent />
        </Suspense>
    }
}
