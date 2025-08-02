pub mod airports;
pub mod flights;
pub mod satellite;

/// API base paths (const str)
pub const AIRPORTS_API_PATH: &str = "/api/v1/airports";
pub const AIRPORTS_SEARCH_API_PATH: &str = "/api/v1/airports/search";
pub const FLIGHTS_API_PATH: &str = "/api/v1/flights";
pub const SATELLITES_API_PATH: &str = "/api/v1/satellites";
pub const BABEL_API_PATH: &str = "/api/v1/babel";
pub const NEXUS_API_PATH: &str = "/api/v1/nexus";

// For use in OpenAPI documentation
pub const API_BASE_PATH: &str = "/api/v1";
