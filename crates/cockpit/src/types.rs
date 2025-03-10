use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Airport {
    pub code: String,
    pub position: (f64, f64),
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Flight {
    pub flight_number: String,
    pub departure: Airport,
    pub arrival: Airport,
    pub position: Option<(f64, f64)>,
}
