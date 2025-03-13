use crate::components::GrundDisplay;
use crate::FlightWithDelay;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct FlightWithDelayProps {
    pub flight_with_delay: FlightWithDelay,
}

#[function_component(FlightWithDelayComponent)]
pub fn flight_with_delay(props: &FlightWithDelayProps) -> Html {
    let flight = &props.flight_with_delay.flight;

    html! {
        <div class="flight-item">
            <div class="flight-info">
                <h3>{&flight.flight_number}</h3>
                <p class="route">
                    {&flight.departure}
                    {" → "}
                    {&flight.arrival}
                </p>
                <p class="time">
                    {"Departure: "}
                    {flight.departure_time.format("%Y-%m-%d %H:%M")}
                </p>
                if let Some(arrival_time) = flight.arrival_time {
                    <p class="time">
                        {"Arrival: "}
                        {arrival_time.format("%Y-%m-%d %H:%M")}
                    </p>
                }
            </div>
            <GrundDisplay grund={props.flight_with_delay.grund.clone()} />
        </div>
    }
}
