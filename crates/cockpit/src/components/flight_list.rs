use gloo_net::http::Request;
use sky_tracer::protocol::flights::FlightResponse;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[function_component(FlightList)]
pub fn flight_list() -> Html {
    let flights = use_state(Vec::<FlightResponse>::new);
    let loading = use_state(|| true);

    {
        let flights = flights.clone();
        let loading = loading.clone();

        use_effect_with((), move |_| {
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
            || ()
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
                        </tr>
                    </thead>
                    <tbody>
                        {for flights.iter().map(|flight| html! {
                            <tr>
                                <td>{&flight.flight_number}</td>
                                <td>{&flight.aircraft_number}</td>
                                <td>{&flight.departure}</td>
                                <td>{&flight.arrival}</td>
                                <td>{flight.departure_time.format("%Y-%m-%d %H:%M").to_string()}</td>
                            </tr>
                        })}
                    </tbody>
                </table>
            }
        </div>
    }
}
