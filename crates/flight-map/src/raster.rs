use crate::geo::{SVG_HEIGHT, SVG_WIDTH};

/// Rasterize an SVG string to PNG bytes using resvg + tiny-skia.
pub fn rasterize(svg: &str) -> Result<Vec<u8>, String> {
    let width = SVG_WIDTH as u32;
    let height = SVG_HEIGHT as u32;

    let options = resvg::usvg::Options::default();
    let tree = resvg::usvg::Tree::from_str(svg, &options)
        .map_err(|e| format!("SVG parse error: {e}"))?;

    let mut pixmap = tiny_skia::Pixmap::new(width, height)
        .ok_or("Failed to allocate pixmap")?;

    resvg::render(
        &tree,
        tiny_skia::Transform::default(),
        &mut pixmap.as_mut(),
    );

    pixmap.encode_png().map_err(|e| format!("PNG encode error: {e}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{render_flight_map, AirportPin};

    #[test]
    fn png_starts_with_png_header() {
        let svg = render_flight_map(vec![], vec![], None);
        let png = rasterize(&svg).expect("rasterize failed");
        assert_eq!(&png[..8], b"\x89PNG\r\n\x1a\n");
    }

    #[test]
    fn png_has_correct_dimensions() {
        let airports = vec![AirportPin {
            code: "FRA".to_string(),
            lat: 50.033,
            lon: 8.570,
        }];
        let svg = render_flight_map(airports, vec![], Some("Test".to_string()));
        let png = rasterize(&svg).expect("rasterize failed");

        let decoder = png::Decoder::new(std::io::Cursor::new(&png));
        let reader = decoder.read_info().expect("read png info");
        assert_eq!(reader.info().width, SVG_WIDTH as u32);
        assert_eq!(reader.info().height, SVG_HEIGHT as u32);
    }
}
