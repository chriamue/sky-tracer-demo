use thiserror::Error;

/// An error that can occur when working with airports
#[derive(Error, Debug)]
pub enum AirportError {
    #[error("Airport not found with code: {0}")]
    NotFound(String),

    #[error("CSV parsing error: {0}")]
    CsvError(#[from] csv::Error),

    #[error("Missing required field: {0}")]
    MissingField(String),

    #[error("Invalid UTF-8 in {field}: {message}")]
    InvalidUtf8 { field: String, message: String },

    #[error("Invalid value for {field}: {value}")]
    InvalidValue { field: String, value: String },

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Error loading airports: {0}")]
    LoadError(String),
}

impl From<std::string::FromUtf8Error> for AirportError {
    fn from(err: std::string::FromUtf8Error) -> Self {
        AirportError::InvalidUtf8 {
            field: "unknown".to_string(),
            message: err.to_string(),
        }
    }
}

impl AirportError {
    pub fn missing_field(field: impl Into<String>) -> Self {
        Self::MissingField(field.into())
    }

    pub fn invalid_utf8(field: impl Into<String>, message: impl Into<String>) -> Self {
        Self::InvalidUtf8 {
            field: field.into(),
            message: message.into(),
        }
    }

    pub fn invalid_value(field: impl Into<String>, value: impl Into<String>) -> Self {
        Self::InvalidValue {
            field: field.into(),
            value: value.into(),
        }
    }
}
