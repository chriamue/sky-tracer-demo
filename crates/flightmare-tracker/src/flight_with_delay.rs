use super::grund::get_random_grund;
use super::Grund;
use sky_tracer::protocol::flights::FlightResponse;

#[derive(Clone, PartialEq)]
pub struct FlightWithDelay {
    pub flight: FlightResponse,
    pub grund: Option<Grund>,
}

impl FlightWithDelay {
    pub fn with_grund(flight: FlightResponse, grund: Grund) -> Self {
        Self {
            flight,
            grund: Some(grund),
        }
    }
}

impl From<FlightResponse> for FlightWithDelay {
    fn from(flight: FlightResponse) -> Self {
        FlightWithDelay {
            flight,
            grund: Some(get_random_grund()),
        }
    }
}
