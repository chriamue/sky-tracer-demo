use crate::geo::{lat_to_y, lon_to_x};
use geo_svg::{Color, ToSvg};
use geo_types::{coord, LineString, Polygon};
use geojson::{GeoJson, GeometryValue, Position};

const LAND_GEOJSON: &str = include_str!("data/ne_110m_land.geojson");

/// Returns SVG `<path>` elements for all continental landmasses,
/// projected into SVG coordinate space via equirectangular projection.
pub fn continent_svg_elements() -> String {
    let gj: GeoJson = LAND_GEOJSON.parse().expect("valid GeoJSON");
    let mut out = String::new();

    if let GeoJson::FeatureCollection(fc) = gj {
        for feature in fc.features {
            if let Some(geometry) = feature.geometry {
                match geometry.value {
                    GeometryValue::Polygon { coordinates } => {
                        out.push_str(&polygon_to_path(&coordinates));
                        out.push('\n');
                    }
                    GeometryValue::MultiPolygon { coordinates } => {
                        for rings in &coordinates {
                            out.push_str(&polygon_to_path(rings));
                            out.push('\n');
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    out
}

fn polygon_to_path(rings: &[Vec<Position>]) -> String {
    if rings.is_empty() {
        return String::new();
    }

    let exterior: LineString<f64> = rings[0]
        .iter()
        .map(|p| coord! { x: lon_to_x(p[0]), y: lat_to_y(p[1]) })
        .collect();

    let interiors: Vec<LineString<f64>> = rings[1..]
        .iter()
        .map(|ring| {
            ring.iter()
                .map(|p| coord! { x: lon_to_x(p[0]), y: lat_to_y(p[1]) })
                .collect()
        })
        .collect();

    let polygon = Polygon::new(exterior, interiors);

    let svg_str = polygon
        .to_svg()
        .with_fill_color(Color::Rgb(26, 58, 42))
        .with_stroke_color(Color::Rgb(42, 90, 58))
        .with_stroke_width(0.5)
        .to_string();

    extract_path_elements(&svg_str)
}

/// Extracts all `<path .../>` elements from a geo-svg output string.
fn extract_path_elements(svg: &str) -> String {
    let mut result = String::new();
    let mut remaining = svg;

    while let Some(start) = remaining.find("<path") {
        let rest = &remaining[start..];
        if let Some(end) = rest.find("/>") {
            result.push_str(&rest[..end + 2]);
            remaining = &rest[end + 2..];
        } else {
            break;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_many_polygon_paths() {
        let elements = continent_svg_elements();
        assert!(elements.contains("<path"));
        let count = elements.matches("<path").count();
        assert!(count > 50, "expected many land polygons, got {count}");
    }

    #[test]
    fn paths_have_fill_attribute() {
        let elements = continent_svg_elements();
        assert!(elements.contains("fill="));
    }
}
