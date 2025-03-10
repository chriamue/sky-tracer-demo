use crate::utils::calculate_distance;
use chrono::Utc;
use sky_tracer::protocol::flights::{FlightPositionResponse, FlightResponse};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct DelayTableProps {
    pub flights: Vec<(FlightResponse, Option<FlightPositionResponse>)>,
    pub airport_position: Option<(f64, f64)>,
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
                        <th>{"Distance"}</th>
                    </tr>
                </thead>
                <tbody>
                    {props.flights.iter().map(|(flight, position)| {
                        let status = calculate_delay_status(flight);
                        let distance = calculate_flight_distance(position, props.airport_position);
                        html! {
                            <tr>
                                <td>{&flight.flight_number}</td>
                                <td>{&flight.departure}</td>
                                <td>{&flight.arrival}</td>
                                <td>{flight.departure_time.format("%H:%M").to_string()}</td>
                                <td>
                                    <span class={status.1}>{status.0}</span>
                                </td>
                                <td>
                                    {format_distance(distance)}
                                </td>
                            </tr>
                        }
                    }).collect::<Html>()}
                </tbody>
            </table>
        }
    }
}

fn calculate_flight_distance(
    position: &Option<FlightPositionResponse>,
    airport_position: Option<(f64, f64)>,
) -> Option<f64> {
    match (position, airport_position) {
        (Some(pos), Some((airport_lat, airport_lon))) => Some(calculate_distance(
            pos.latitude,
            pos.longitude,
            airport_lat,
            airport_lon,
        )),
        _ => None,
    }
}

fn format_distance(distance: Option<f64>) -> String {
    distance
        .map(|d| format!("{:.1} km", d))
        .unwrap_or_else(|| "N/A".to_string())
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
