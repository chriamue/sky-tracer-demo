pub mod model;

#[cfg(feature = "protocol")]
pub mod protocol;

#[cfg(feature = "telemetry")]
pub mod telemetry;

pub mod prelude {
    pub use crate::model::airport::Airport;
    #[cfg(feature = "telemetry")]
    pub use crate::telemetry::*;
}
