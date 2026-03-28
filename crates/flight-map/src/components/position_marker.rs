use crate::geo::{lat_to_y, lon_to_x};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PositionMarkerProps {
    pub lat: f64,
    pub lon: f64,
}

#[function_component]
pub fn PositionMarker(props: &PositionMarkerProps) -> Html {
    let px = format!("{:.1}", lon_to_x(props.lon));
    let py = format!("{:.1}", lat_to_y(props.lat));

    html! {
        <g>
            <circle cx={px.clone()} cy={py.clone()} r="6"
                    fill="#fbbf24" fill-opacity="0.25"
                    stroke="#fbbf24" stroke-width="1" />
            <circle cx={px} cy={py} r="3" fill="#fbbf24" />
        </g>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use yew::ServerRenderer;

    #[tokio::test]
    async fn renders_two_circles() {
        let html = ServerRenderer::<PositionMarker>::with_props(|| PositionMarkerProps {
            lat: 48.8566,
            lon: 2.3522,
        })
        .render()
        .await;
        assert_eq!(html.matches("<circle").count(), 2);
        assert!(html.contains("#fbbf24"));
    }
}
