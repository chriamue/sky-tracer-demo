/// A single airport pin on the map.
#[derive(Clone, Debug)]
pub struct AirportPin {
    pub code: String,
    pub lat: f64,
    pub lon: f64,
}

/// A flight route arc with an optional current position.
#[derive(Clone, Debug)]
pub struct RouteArc {
    pub label: String,
    pub dep_lat: f64,
    pub dep_lon: f64,
    pub arr_lat: f64,
    pub arr_lon: f64,
    pub pos_lat: Option<f64>,
    pub pos_lon: Option<f64>,
}
