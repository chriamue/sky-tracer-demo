pub mod airports;
pub mod flights;
pub mod satellite;

/// API base paths (const str)
pub const AIRPORTS_API_PATH: &str = "/api/v1/airports";
pub const AIRPORTS_SEARCH_API_PATH: &str = "/api/v1/airports/search";
pub const FLIGHTS_API_PATH: &str = "/api/v1/flights";
pub const FLIGHTS_POSITION_API_PATH: &str = "/api/v1/flights/{flight_number}/position";
pub const SATELLITES_API_PATH: &str = "/api/v1/satellites";
pub const SATELLITES_POSITION_API_PATH: &str = "/api/v1/satellites/position";
pub const SATELLITES_STATUS_API_PATH: &str = "/api/v1/satellites/{id}/status";
pub const BABEL_API_PATH: &str = "/api/v1/babel";
pub const BABEL_AIRPORT_API_PATH: &str = "/api/v1/babel/{airport_code}";
pub const BABEL_POSITION_API_PATH: &str = "/api/v1/babel/{flight_number}/position";
pub const ORBITAL_BEACON_POSITION_API_PATH: &str = "/api/v1/position";
pub const ORBITAL_BEACON_SATELLITES_API_PATH: &str = "/api/v1/satellites";
pub const NEXUS_API_PATH: &str = "/api/v1/nexus";
pub const NEXUS_AIRPORTS_API_PATH: &str = "/api/v1/nexus/airports";
pub const NEXUS_FLIGHTS_API_PATH: &str = "/api/v1/nexus/flights";
pub const NEXUS_SATELLITES_API_PATH: &str = "/api/v1/nexus/satellites";

// For use in OpenAPI documentation
pub const API_BASE_PATH: &str = "/api/v1";
