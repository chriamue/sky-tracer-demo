pub mod components;
mod geo;
mod raster;

pub use components::{AirportPin, FlightMapProps, FlightMapSvg, RouteArc};
pub use raster::rasterize;

/// Render the flight map SVG to a string using Yew SSR.
///
/// `hydratable(false)` disables SSR hydration markers so the output is
/// clean, standalone SVG with no `<!--<[]>-->` comments.
pub async fn render_flight_map(
    airports: Vec<AirportPin>,
    routes: Vec<RouteArc>,
    title: Option<String>,
) -> String {
    use yew::ServerRenderer;

    ServerRenderer::<FlightMapSvg>::with_props(move || FlightMapProps {
        airports,
        routes,
        title,
    })
    .hydratable(false)
    .render()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fra() -> AirportPin {
        AirportPin { code: "FRA".to_string(), lat: 50.033, lon: 8.570 }
    }

    fn jfk() -> AirportPin {
        AirportPin { code: "JFK".to_string(), lat: 40.639, lon: -73.778 }
    }

    fn lh400() -> RouteArc {
        RouteArc {
            label: "LH400".to_string(),
            dep_lat: fra().lat,
            dep_lon: fra().lon,
            arr_lat: jfk().lat,
            arr_lon: jfk().lon,
            pos_lat: None,
            pos_lon: None,
        }
    }

    #[tokio::test]
    async fn svg_root_element_present() {
        let svg = render_flight_map(vec![], vec![], None).await;
        assert!(svg.contains("<svg"));
        assert!(svg.contains("</svg>"));
    }

    #[tokio::test]
    async fn airport_codes_appear_in_output() {
        let svg = render_flight_map(vec![fra(), jfk()], vec![], None).await;
        assert!(svg.contains("FRA"));
        assert!(svg.contains("JFK"));
    }

    #[tokio::test]
    async fn route_arc_produces_path_elements() {
        let svg = render_flight_map(vec![fra(), jfk()], vec![lh400()], None).await;
        assert!(svg.contains("<path"));
    }

    #[tokio::test]
    async fn title_appears_when_provided() {
        let svg =
            render_flight_map(vec![], vec![], Some("Test Map".to_string())).await;
        assert!(svg.contains("Test Map"));
    }

    #[tokio::test]
    async fn position_marker_rendered_when_in_flight() {
        let route = RouteArc {
            pos_lat: Some(48.0),
            pos_lon: Some(-20.0),
            ..lh400()
        };
        let svg = render_flight_map(vec![fra(), jfk()], vec![route], None).await;
        assert!(svg.contains("#fbbf24"));
    }

    #[tokio::test]
    async fn empty_map_has_continent_paths_but_no_routes() {
        let svg = render_flight_map(vec![], vec![], None).await;
        // Continents produce paths; no flight routes or airport code labels
        assert!(svg.contains("<path"));
        assert!(!svg.contains("stroke-dasharray"));
        assert!(!svg.contains("monospace\">F")); // no airport code labels
    }
}
