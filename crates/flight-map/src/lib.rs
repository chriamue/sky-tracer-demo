mod continents;
mod geo;
mod map;
mod raster;
mod types;

pub use raster::rasterize;
pub use types::{AirportPin, RouteArc};

/// Render the flight map to an SVG string.
pub fn render_flight_map(
    airports: Vec<AirportPin>,
    routes: Vec<RouteArc>,
    title: Option<String>,
) -> String {
    map::render(airports, routes, title)
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

    #[test]
    fn svg_root_element_present() {
        let svg = render_flight_map(vec![], vec![], None);
        assert!(svg.contains("<svg"));
        assert!(svg.contains("</svg>"));
    }

    #[test]
    fn airport_codes_appear_in_output() {
        let svg = render_flight_map(vec![fra(), jfk()], vec![], None);
        assert!(svg.contains("FRA"));
        assert!(svg.contains("JFK"));
    }

    #[test]
    fn route_arc_produces_path_elements() {
        let svg = render_flight_map(vec![fra(), jfk()], vec![lh400()], None);
        assert!(svg.contains("<path"));
    }

    #[test]
    fn title_appears_when_provided() {
        let svg = render_flight_map(vec![], vec![], Some("Test Map".to_string()));
        assert!(svg.contains("Test Map"));
    }

    #[test]
    fn position_marker_rendered_when_in_flight() {
        let route = RouteArc {
            pos_lat: Some(48.0),
            pos_lon: Some(-20.0),
            ..lh400()
        };
        let svg = render_flight_map(vec![fra(), jfk()], vec![route], None);
        assert!(svg.contains("#fbbf24"));
    }

    #[test]
    fn empty_map_has_continent_paths_but_no_routes() {
        let svg = render_flight_map(vec![], vec![], None);
        assert!(svg.contains("<path"));
        assert!(!svg.contains("stroke-dasharray"));
    }
}
