use crate::components::types::AirportPin;
use crate::geo::{lat_to_y, lon_to_x};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct AirportPinProps {
    pub pin: AirportPin,
}

#[function_component]
pub fn AirportPinEl(props: &AirportPinProps) -> Html {
    let cx = format!("{:.1}", lon_to_x(props.pin.lon));
    let cy = format!("{:.1}", lat_to_y(props.pin.lat));
    let tx = format!("{:.1}", lon_to_x(props.pin.lon) + 7.0);
    let ty = format!("{:.1}", lat_to_y(props.pin.lat) - 5.0);
    let code = props.pin.code.clone();

    html! {
        <g>
            <circle cx={cx.clone()} cy={cy.clone()} r="5"
                    fill="#22d3ee" fill-opacity="0.15"
                    stroke="#22d3ee" stroke-width="1.5" />
            <circle cx={cx} cy={cy} r="2" fill="#22d3ee" />
            <text x={tx} y={ty} font-size="9" fill="#94a3b8"
                  font-family="monospace">{ code }</text>
        </g>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use yew::ServerRenderer;

    #[tokio::test]
    async fn renders_iata_code() {
        let html = ServerRenderer::<AirportPinEl>::with_props(|| AirportPinProps {
            pin: AirportPin {
                code: "FRA".to_string(),
                lat: 50.033,
                lon: 8.570,
            },
        })
        .render()
        .await;
        assert!(html.contains("FRA"));
    }

    #[tokio::test]
    async fn renders_circles() {
        let html = ServerRenderer::<AirportPinEl>::with_props(|| AirportPinProps {
            pin: AirportPin {
                code: "LHR".to_string(),
                lat: 51.477,
                lon: -0.461,
            },
        })
        .render()
        .await;
        assert_eq!(html.matches("<circle").count(), 2);
    }
}
