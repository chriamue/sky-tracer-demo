use crate::components::position_marker::PositionMarker;
use crate::components::types::RouteArc;
use crate::geo::arc_path;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct RouteArcProps {
    pub route: RouteArc,
}

#[function_component]
pub fn RouteArcEl(props: &RouteArcProps) -> Html {
    let r = &props.route;
    let path = arc_path(r.dep_lon, r.dep_lat, r.arr_lon, r.arr_lat);

    let position = match (r.pos_lat, r.pos_lon) {
        (Some(lat), Some(lon)) => html! { <PositionMarker {lat} {lon} /> },
        _ => html! {},
    };

    html! {
        <g>
            <path d={path.clone()} fill="none"
                  stroke="#3b82f6" stroke-width="2"
                  stroke-opacity="0.4" stroke-dasharray="4 2" />
            <path d={path} fill="none"
                  stroke="#60a5fa" stroke-width="1"
                  stroke-opacity="0.8" />
            { position }
        </g>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use yew::ServerRenderer;

    #[tokio::test]
    async fn renders_two_paths() {
        let html = ServerRenderer::<RouteArcEl>::with_props(|| RouteArcProps {
            route: RouteArc {
                label: "LH400".to_string(),
                dep_lat: 50.0,
                dep_lon: 8.5,
                arr_lat: 40.6,
                arr_lon: -73.7,
                pos_lat: None,
                pos_lon: None,
            },
        })
        .render()
        .await;
        assert_eq!(html.matches("<path").count(), 2);
        assert!(!html.contains("<circle"));
    }

    #[tokio::test]
    async fn renders_position_marker_when_present() {
        let html = ServerRenderer::<RouteArcEl>::with_props(|| RouteArcProps {
            route: RouteArc {
                label: "BA112".to_string(),
                dep_lat: 51.4,
                dep_lon: -0.4,
                arr_lat: 40.6,
                arr_lon: -73.7,
                pos_lat: Some(48.0),
                pos_lon: Some(-20.0),
            },
        })
        .render()
        .await;
        assert!(html.contains("<circle"));
        assert!(html.contains("#fbbf24"));
    }
}
