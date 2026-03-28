pub const SVG_WIDTH: f64 = 1000.0;
pub const SVG_HEIGHT: f64 = 500.0;
pub const PAD: f64 = 30.0;

/// Equirectangular projection: longitude → x
pub fn lon_to_x(lon: f64) -> f64 {
    PAD + (lon + 180.0) / 360.0 * (SVG_WIDTH - 2.0 * PAD)
}

/// Equirectangular projection: latitude → y
pub fn lat_to_y(lat: f64) -> f64 {
    PAD + (90.0 - lat) / 180.0 * (SVG_HEIGHT - 2.0 * PAD)
}

/// Quadratic bezier arc path between two geo-points.
/// The control point is lifted above the midpoint proportional to distance.
pub fn arc_path(dep_lon: f64, dep_lat: f64, arr_lon: f64, arr_lat: f64) -> String {
    let x1 = lon_to_x(dep_lon);
    let y1 = lat_to_y(dep_lat);
    let x2 = lon_to_x(arr_lon);
    let y2 = lat_to_y(arr_lat);
    let mx = (x1 + x2) / 2.0;
    let my = (y1 + y2) / 2.0;
    let dist = ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt();
    let cpy = my - dist * 0.3;
    format!("M {x1:.1} {y1:.1} Q {mx:.1} {cpy:.1} {x2:.1} {y2:.1}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lon_to_x_left_edge() {
        assert_eq!(lon_to_x(-180.0), PAD);
    }

    #[test]
    fn lon_to_x_right_edge() {
        assert_eq!(lon_to_x(180.0), SVG_WIDTH - PAD);
    }

    #[test]
    fn lon_to_x_center() {
        assert_eq!(lon_to_x(0.0), SVG_WIDTH / 2.0);
    }

    #[test]
    fn lat_to_y_top() {
        assert_eq!(lat_to_y(90.0), PAD);
    }

    #[test]
    fn lat_to_y_bottom() {
        assert_eq!(lat_to_y(-90.0), SVG_HEIGHT - PAD);
    }

    #[test]
    fn lat_to_y_equator() {
        assert_eq!(lat_to_y(0.0), SVG_HEIGHT / 2.0);
    }

    #[test]
    fn arc_path_format() {
        let path = arc_path(0.0, 0.0, 10.0, 10.0);
        assert!(path.starts_with("M "));
        assert!(path.contains(" Q "));
    }

    #[test]
    fn arc_path_same_point_has_zero_dist() {
        let path = arc_path(8.0, 50.0, 8.0, 50.0);
        // control point y == midpoint y when dist == 0
        let parts: Vec<&str> = path.split_whitespace().collect();
        // "M x1 y1 Q cpx cpy x2 y2"  →  indices: 0=M 1=x1 2=y1 3=Q 4=cpx 5=cpy 6=x2 7=y2
        let cpy: f64 = parts[5].parse().unwrap();
        let y1: f64 = parts[2].parse().unwrap();
        assert!((cpy - y1).abs() < 0.01);
    }
}
