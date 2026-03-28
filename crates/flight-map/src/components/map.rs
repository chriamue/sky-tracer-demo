use crate::components::{
    airport_pin::AirportPinEl,
    continents::Continents,
    grid::Grid,
    legend::Legend,
    route_arc::RouteArcEl,
    title::Title,
    types::{AirportPin, RouteArc},
};
use crate::geo::{PAD, SVG_HEIGHT, SVG_WIDTH};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct FlightMapProps {
    pub airports: Vec<AirportPin>,
    pub routes: Vec<RouteArc>,
    pub title: Option<String>,
}

#[function_component]
pub fn FlightMapSvg(props: &FlightMapProps) -> Html {
    let w = format!("{SVG_WIDTH}");
    let h = format!("{SVG_HEIGHT}");
    let vb = format!("0 0 {SVG_WIDTH} {SVG_HEIGHT}");
    let airport_count = props.airports.len();
    let route_count = props.routes.len();

    html! {
        <svg xmlns="http://www.w3.org/2000/svg"
             width={w.clone()} height={h.clone()} viewBox={vb}>

            <rect width={w} height={h} fill="#0a0f1e" />
            <rect x={format!("{PAD}")} y={format!("{PAD}")}
                  width={format!("{:.1}", SVG_WIDTH - 2.0 * PAD)}
                  height={format!("{:.1}", SVG_HEIGHT - 2.0 * PAD)}
                  fill="#0d1b2e" stroke="#1e3a5f" stroke-width="0.5" />

            <Grid />
            <Continents />

            <g>
                { for props.routes.iter().map(|r| html! {
                    <RouteArcEl route={r.clone()} />
                }) }
            </g>

            <g>
                { for props.airports.iter().map(|a| html! {
                    <AirportPinEl pin={a.clone()} />
                }) }
            </g>

            <Title text={props.title.clone()} />
            <Legend {airport_count} {route_count} />

        </svg>
    }
}
