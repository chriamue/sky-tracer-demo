use sky_tracer::protocol::flights::FlightResponse;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct FlightTableProps {
    pub flights: Vec<FlightResponse>,
}

#[function_component(FlightTable)]
pub fn flight_table(props: &FlightTableProps) -> Html {
    html! {
        <table class="flight-table">
            <thead>
                <tr>
                    <th>{"Flight #"}</th>
                    <th>{"Aircraft"}</th>
                    <th>{"From"}</th>
                    <th>{"To"}</th>
                    <th>{"Departure Time"}</th>
                </tr>
            </thead>
            <tbody>
                {props.flights.iter().map(|flight| {
                    html! {
                        <tr>
                            <td>{&flight.flight_number}</td>
                            <td>{&flight.aircraft_number}</td>
                            <td>{&flight.departure}</td>
                            <td>{&flight.arrival}</td>
                            <td>{flight.departure_time.format("%Y-%m-%d %H:%M").to_string()}</td>
                        </tr>
                    }
                }).collect::<Html>()}
            </tbody>
        </table>
    }
}
