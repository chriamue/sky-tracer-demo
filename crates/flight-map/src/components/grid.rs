use crate::geo::{PAD, SVG_HEIGHT, SVG_WIDTH, lat_to_y, lon_to_x};
use yew::prelude::*;

#[function_component]
pub fn Grid() -> Html {
    let map_x2 = format!("{:.1}", SVG_WIDTH - PAD);
    let map_y2 = format!("{:.1}", SVG_HEIGHT - PAD);
    let pad = format!("{PAD}");

    let lat_lines: Vec<Html> = (-3i32..=3)
        .map(|i| {
            let lat = i as f64 * 30.0;
            let y = format!("{:.1}", lat_to_y(lat));
            let (stroke, sw) = if i == 0 {
                ("#2a4a8f", "1")
            } else {
                ("#1a2f4a", "0.5")
            };
            html! {
                <line x1={pad.clone()} y1={y.clone()}
                      x2={map_x2.clone()} y2={y}
                      stroke={stroke} stroke-width={sw} />
            }
        })
        .collect();

    let lon_lines: Vec<Html> = (-6i32..=6)
        .map(|i| {
            let lon = i as f64 * 30.0;
            let x = format!("{:.1}", lon_to_x(lon));
            html! {
                <line x1={x.clone()} y1={pad.clone()}
                      x2={x} y2={map_y2.clone()}
                      stroke="#1a2f4a" stroke-width="0.5" />
            }
        })
        .collect();

    html! {
        <g>
            { for lat_lines }
            { for lon_lines }
        </g>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use yew::ServerRenderer;

    #[tokio::test]
    async fn renders_line_elements() {
        let html = ServerRenderer::<Grid>::new().render().await;
        assert!(html.contains("<line"));
    }

    #[tokio::test]
    async fn renders_equator_highlighted() {
        let html = ServerRenderer::<Grid>::new().render().await;
        assert!(html.contains("#2a4a8f"));
    }

    #[tokio::test]
    async fn renders_expected_line_count() {
        let html = ServerRenderer::<Grid>::new().render().await;
        // 7 lat lines (-3..=3) + 13 lon lines (-6..=6) = 20 lines
        let count = html.matches("<line").count();
        assert_eq!(count, 20);
    }
}
