use crate::geo::{PAD, SVG_HEIGHT, SVG_WIDTH};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct LegendProps {
    pub airport_count: usize,
    pub route_count: usize,
}

#[function_component]
pub fn Legend(props: &LegendProps) -> Html {
    let lx = format!("{:.1}", PAD + 8.0);
    let stats_x = format!("{:.1}", SVG_WIDTH - PAD);
    let bottom_y = format!("{:.1}", SVG_HEIGHT - 8.0);
    let ap_cy = format!("{:.1}", SVG_HEIGHT - PAD - 30.0);
    let ap_label_y = format!("{:.1}", SVG_HEIGHT - PAD - 26.0);
    let pos_cy = format!("{:.1}", SVG_HEIGHT - PAD - 14.0);
    let pos_label_y = format!("{:.1}", SVG_HEIGHT - PAD - 10.0);
    let label_x = format!("{:.1}", PAD + 16.0);

    html! {
        <g>
            <circle cx={lx.clone()} cy={ap_cy} r="4"
                    fill="#22d3ee" fill-opacity="0.2"
                    stroke="#22d3ee" stroke-width="1.5" />
            <text x={label_x.clone()} y={ap_label_y}
                  font-size="8" fill="#64748b" font-family="monospace">
                { "Airport" }
            </text>
            <circle cx={lx} cy={pos_cy} r="3" fill="#fbbf24" />
            <text x={label_x} y={pos_label_y}
                  font-size="8" fill="#64748b" font-family="monospace">
                { "In-flight" }
            </text>
            <text x={stats_x} y={bottom_y} text-anchor="end"
                  font-size="8" fill="#334155" font-family="monospace">
                { format!("{} airports · {} routes", props.airport_count, props.route_count) }
            </text>
        </g>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use yew::ServerRenderer;

    #[tokio::test]
    async fn renders_airport_and_inflight_labels() {
        let html = ServerRenderer::<Legend>::with_props(|| LegendProps {
            airport_count: 3,
            route_count: 2,
        })
        .render()
        .await;
        assert!(html.contains("Airport"));
        assert!(html.contains("In-flight"));
    }

    #[tokio::test]
    async fn renders_stats() {
        let html = ServerRenderer::<Legend>::with_props(|| LegendProps {
            airport_count: 5,
            route_count: 4,
        })
        .render()
        .await;
        assert!(html.contains("5 airports"));
        assert!(html.contains("4 routes"));
    }
}
