pub mod model;

#[cfg(feature = "protocol")]
pub mod protocol;

pub mod prelude {
    pub use crate::model::airport::Airport;
}
