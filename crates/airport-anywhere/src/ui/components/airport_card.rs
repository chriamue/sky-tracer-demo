use sky_tracer::protocol::airports::AirportResponse;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct AirportCardProps {
    pub airport: AirportResponse,
}

#[function_component(AirportCard)]
pub fn airport_card(props: &AirportCardProps) -> Html {
    html! {
        <div class="airport-card">
            <h3 class="airport-name">{&props.airport.name}</h3>
            <div class="airport-codes">
                {if !props.airport.code.is_empty() {
                    format!("IATA: {}", props.airport.code)
                } else {
                    "No IATA".to_string()
                }}
                {" / "}
                {if !props.airport.code.is_empty() {
                    format!("ICAO: {}", props.airport.code)
                } else {
                    "No ICAO".to_string()
                }}
            </div>
            <div class="airport-position">
                {"Position: "}
                {format!("{:.6}°N, {:.6}°E",
                    props.airport.position.latitude,
                    props.airport.position.longitude)}
            </div>
        </div>
    }
}
