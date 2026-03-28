use crate::continents::continent_svg_elements;
use crate::geo::{arc_path, lat_to_y, lon_to_x, PAD, SVG_HEIGHT, SVG_WIDTH};
use crate::types::{AirportPin, RouteArc};

pub fn render(airports: Vec<AirportPin>, routes: Vec<RouteArc>, title: Option<String>) -> String {
    let inner_w = SVG_WIDTH - 2.0 * PAD;
    let inner_h = SVG_HEIGHT - 2.0 * PAD;

    let grid = render_grid();
    let continents = continent_svg_elements();
    let route_arcs = render_routes(&routes);
    let airport_pins = render_airports(&airports);
    let positions = render_positions(&routes);
    let title_el = render_title(title.as_deref());
    let legend_el = render_legend(airports.len(), routes.len());

    format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{SVG_WIDTH}\" height=\"{SVG_HEIGHT}\" viewBox=\"0 0 {SVG_WIDTH} {SVG_HEIGHT}\">\n\
<rect width=\"{SVG_WIDTH}\" height=\"{SVG_HEIGHT}\" fill=\"#0a0f1e\"/>\n\
<rect x=\"{PAD}\" y=\"{PAD}\" width=\"{inner_w:.1}\" height=\"{inner_h:.1}\" fill=\"#0d1b2e\" stroke=\"#1e3a5f\" stroke-width=\"0.5\"/>\n\
{grid}\n\
<g id=\"continents\">{continents}</g>\n\
<g id=\"routes\">{route_arcs}</g>\n\
<g id=\"airports\">{airport_pins}</g>\n\
<g id=\"positions\">{positions}</g>\n\
{title_el}\n\
{legend_el}\n\
</svg>"
    )
}

fn render_grid() -> String {
    let mut out = String::from(
        "<g id=\"grid\" stroke=\"#1e3a5f\" stroke-width=\"0.3\" opacity=\"0.7\">",
    );

    for lon in (-180..=180_i32).step_by(30) {
        let x = lon_to_x(lon as f64);
        out.push_str(&format!(
            "<line x1=\"{x:.1}\" y1=\"{PAD}\" x2=\"{x:.1}\" y2=\"{:.1}\"/>",
            SVG_HEIGHT - PAD
        ));
    }

    for lat in (-90..=90_i32).step_by(30) {
        let y = lat_to_y(lat as f64);
        let (stroke, width) = if lat == 0 {
            ("#2a5a8f", "0.6")
        } else {
            ("#1e3a5f", "0.3")
        };
        out.push_str(&format!(
            "<line x1=\"{PAD}\" y1=\"{y:.1}\" x2=\"{:.1}\" y2=\"{y:.1}\" stroke=\"{stroke}\" stroke-width=\"{width}\"/>",
            SVG_WIDTH - PAD
        ));
    }

    out.push_str("</g>");
    out
}

fn render_routes(routes: &[RouteArc]) -> String {
    routes
        .iter()
        .map(|r| {
            let d = arc_path(r.dep_lon, r.dep_lat, r.arr_lon, r.arr_lat);
            format!(
                "<path d=\"{d}\" fill=\"none\" stroke=\"#3b82f6\" stroke-width=\"1.2\" stroke-dasharray=\"4 3\" opacity=\"0.8\"/>"
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn render_airports(airports: &[AirportPin]) -> String {
    airports
        .iter()
        .map(|a| {
            let x = lon_to_x(a.lon);
            let y = lat_to_y(a.lat);
            let label_y = y - 5.0;
            let code = &a.code;
            format!(
                "<circle cx=\"{x:.1}\" cy=\"{y:.1}\" r=\"3\" fill=\"#60a5fa\" stroke=\"#1e3a5f\" stroke-width=\"0.5\"/>\
<text x=\"{x:.1}\" y=\"{label_y:.1}\" font-size=\"7\" fill=\"#93c5fd\" font-family=\"monospace\" text-anchor=\"middle\">{code}</text>"
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn render_positions(routes: &[RouteArc]) -> String {
    routes
        .iter()
        .filter_map(|r| r.pos_lat.zip(r.pos_lon))
        .map(|(lat, lon)| {
            let x = lon_to_x(lon);
            let y = lat_to_y(lat);
            format!(
                "<circle cx=\"{x:.1}\" cy=\"{y:.1}\" r=\"3\" fill=\"#fbbf24\" stroke=\"#92400e\" stroke-width=\"0.5\"/>"
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn render_title(title: Option<&str>) -> String {
    match title {
        Some(t) => {
            let x = SVG_WIDTH / 2.0;
            let y = PAD - 8.0;
            format!(
                "<text x=\"{x:.1}\" y=\"{y:.1}\" font-size=\"14\" fill=\"#e2e8f0\" font-family=\"sans-serif\" text-anchor=\"middle\" font-weight=\"bold\">{t}</text>"
            )
        }
        None => String::new(),
    }
}

fn render_legend(airport_count: usize, route_count: usize) -> String {
    let box_x = SVG_WIDTH - PAD - 115.0;
    let box_y = SVG_HEIGHT - PAD - 35.0;
    let text_x = box_x + 5.0;
    let text_y1 = box_y + 13.0;
    let text_y2 = box_y + 27.0;

    format!(
        "<rect x=\"{box_x:.1}\" y=\"{box_y:.1}\" width=\"110\" height=\"38\" fill=\"#0a0f1e\" fill-opacity=\"0.8\" rx=\"3\"/>\n\
<text x=\"{text_x:.1}\" y=\"{text_y1:.1}\" font-size=\"8\" fill=\"#94a3b8\" font-family=\"monospace\">&#x25CF; {airport_count} airports</text>\n\
<text x=\"{text_x:.1}\" y=\"{text_y2:.1}\" font-size=\"8\" fill=\"#94a3b8\" font-family=\"monospace\">&#x2015; {route_count} routes</text>"
    )
}
