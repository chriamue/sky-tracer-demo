use crate::geo::{lat_to_y, lon_to_x};
use yew::prelude::*;

// Simplified continent outlines as (longitude, latitude) coordinate arrays.
// Each polygon is a rough but recognisable coastal outline.

const AFRICA: &[(f64, f64)] = &[
    (-6.0, 36.0), (10.0, 38.0), (25.0, 32.0), (32.0, 30.0),
    (37.0, 22.0), (43.0, 12.0), (51.0, 12.0), (44.0, -10.0),
    (36.0, -26.0), (27.0, -35.0), (19.0, -35.0), (15.0, -25.0),
    (11.0, -18.0), (9.0, 2.0), (2.0, 5.0), (-6.0, 5.0),
    (-17.0, 10.0), (-18.0, 15.0), (-18.0, 22.0), (-14.0, 28.0),
    (-8.0, 35.0),
];

const EUROPE: &[(f64, f64)] = &[
    (-10.0, 36.0), (36.0, 36.0), (36.0, 42.0), (28.0, 43.0),
    (24.0, 48.0), (20.0, 56.0), (22.0, 60.0), (25.0, 65.0),
    (18.0, 71.0), (5.0, 72.0), (-5.0, 62.0), (-10.0, 58.0),
    (-8.0, 44.0), (-2.0, 44.0),
];

// Asia simplified — covers Turkey to Japan, Russia to SE Asia.
// Split at the antimeridian: western part stops at 180°, no wrap.
const ASIA: &[(f64, f64)] = &[
    (26.0, 70.0), (60.0, 72.0), (100.0, 75.0), (140.0, 72.0),
    (180.0, 68.0), (180.0, 0.0), (145.0, -5.0), (130.0, -5.0),
    (120.0, 0.0), (105.0, -5.0), (100.0, 2.0), (80.0, 12.0),
    (60.0, 5.0), (44.0, 10.0), (36.0, 36.0), (26.0, 48.0),
];

const NORTH_AMERICA: &[(f64, f64)] = &[
    (-168.0, 55.0), (-140.0, 60.0), (-130.0, 55.0), (-125.0, 49.0),
    (-117.0, 32.0), (-110.0, 23.0), (-90.0, 16.0), (-83.0, 10.0),
    (-77.0, 8.0), (-80.0, 8.0), (-83.0, 10.0), (-87.0, 16.0),
    (-90.0, 21.0), (-97.0, 22.0), (-106.0, 21.0), (-117.0, 32.0),
    (-115.0, 35.0), (-95.0, 46.0), (-83.0, 46.0), (-75.0, 44.0),
    (-70.0, 43.0), (-66.0, 45.0), (-60.0, 47.0), (-53.0, 47.0),
    (-55.0, 52.0), (-60.0, 56.0), (-75.0, 62.0), (-80.0, 65.0),
    (-100.0, 70.0), (-120.0, 70.0), (-140.0, 68.0), (-155.0, 62.0),
    (-168.0, 60.0),
];

const SOUTH_AMERICA: &[(f64, f64)] = &[
    (-80.0, 10.0), (-62.0, 10.0), (-50.0, 2.0), (-35.0, -5.0),
    (-35.0, -23.0), (-50.0, -33.0), (-65.0, -55.0), (-68.0, -55.0),
    (-75.0, -50.0), (-80.0, -38.0), (-75.0, -15.0), (-80.0, 0.0),
];

const AUSTRALIA: &[(f64, f64)] = &[
    (114.0, -22.0), (127.0, -14.0), (137.0, -12.0), (145.0, -15.0),
    (150.0, -22.0), (155.0, -28.0), (150.0, -38.0), (140.0, -39.0),
    (130.0, -34.0), (117.0, -35.0), (114.0, -28.0),
];

const GREENLAND: &[(f64, f64)] = &[
    (-44.0, 60.0), (-24.0, 60.0), (-18.0, 65.0), (-20.0, 70.0),
    (-18.0, 76.0), (-35.0, 84.0), (-55.0, 83.0), (-65.0, 76.0),
    (-55.0, 66.0),
];

const ANTARCTICA: &[(f64, f64)] = &[
    (-180.0, -70.0), (-90.0, -68.0), (0.0, -70.0),
    (90.0, -68.0), (180.0, -70.0), (180.0, -90.0),
    (-180.0, -90.0),
];

fn polygon_to_path(points: &[(f64, f64)]) -> String {
    points
        .iter()
        .enumerate()
        .map(|(i, &(lon, lat))| {
            let x = lon_to_x(lon);
            let y = lat_to_y(lat);
            if i == 0 {
                format!("M {x:.1} {y:.1}")
            } else {
                format!("L {x:.1} {y:.1}")
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
        + " Z"
}

#[function_component]
pub fn Continents() -> Html {
    let landmasses = [
        AFRICA,
        EUROPE,
        ASIA,
        NORTH_AMERICA,
        SOUTH_AMERICA,
        AUSTRALIA,
        GREENLAND,
        ANTARCTICA,
    ];

    let paths: Vec<Html> = landmasses
        .iter()
        .map(|points| {
            let d = polygon_to_path(points);
            html! {
                <path d={d} fill="#1a3a2a" stroke="#2a5a3a" stroke-width="0.5" />
            }
        })
        .collect();

    html! {
        <g>{ for paths }</g>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use yew::ServerRenderer;

    #[tokio::test]
    async fn renders_path_elements() {
        let html = ServerRenderer::<Continents>::new().render().await;
        assert!(html.contains("<path"));
    }

    #[tokio::test]
    async fn renders_all_landmasses() {
        let html = ServerRenderer::<Continents>::new().render().await;
        // 8 landmasses → 8 <path elements
        assert_eq!(html.matches("<path").count(), 8);
    }

    #[test]
    fn polygon_to_path_starts_with_m() {
        let path = polygon_to_path(&[(0.0, 0.0), (10.0, 10.0)]);
        assert!(path.starts_with("M "));
        assert!(path.ends_with('Z'));
    }
}
