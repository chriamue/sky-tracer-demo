use thiserror::Error;

/// An error that can occur when working with airports
#[derive(Error, Debug)]
pub enum AirportError {
    #[error("Airport not found with code: {0}")]
    NotFound(String),

    #[error("Failed to parse airport data: {0}")]
    ParseError(#[from] csv::Error),
}
