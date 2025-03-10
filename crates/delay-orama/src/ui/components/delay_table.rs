use chrono::Utc;
use sky_tracer::protocol::flights::FlightResponse;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct DelayTableProps {
    pub flights: Vec<FlightResponse>,
}

#[function_component(DelayTable)]
pub fn delay_table(props: &DelayTableProps) -> Html {
    html! {
        if props.flights.is_empty() {
            <div class="no-flights">{"No flights found"}</div>
        } else {
            <table class="delay-table">
                <thead>
                    <tr>
                        <th>{"Flight #"}</th>
                        <th>{"From"}</th>
                        <th>{"To"}</th>
                        <th>{"Scheduled"}</th>
                        <th>{"Status"}</th>
                    </tr>
                </thead>
                <tbody>
                    {props.flights.iter().map(|flight| {
                        let status = calculate_delay_status(flight);
                        html! {
                            <tr>
                                <td>{&flight.flight_number}</td>
                                <td>{&flight.departure}</td>
                                <td>{&flight.arrival}</td>
                                <td>{flight.departure_time.format("%H:%M").to_string()}</td>
                                <td>
                                    <span class={status.1}>{status.0}</span>
                                </td>
                            </tr>
                        }
                    }).collect::<Html>()}
                </tbody>
            </table>
        }
    }
}

fn calculate_delay_status(flight: &FlightResponse) -> (String, &'static str) {
    let now = Utc::now();
    if now > flight.departure_time {
        (
            format!(
                "Delayed ({} min)",
                (now - flight.departure_time).num_minutes()
            ),
            "delay-status delayed",
        )
    } else {
        ("On Time".to_string(), "delay-status on-time")
    }
}
