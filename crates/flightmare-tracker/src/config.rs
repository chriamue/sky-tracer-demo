use sky_tracer::protocol::FLIGHTS_API_PATH;

pub struct Config;

impl Config {
    /// Get the flights API endpoint
    pub fn flights_api_url() -> &'static str {
        FLIGHTS_API_PATH
    }

    /// Get the base URL for API requests (can be configured via environment)
    pub fn api_base_url() -> String {
        // In WASM, we use relative URLs that get proxied by nginx
        String::new()
    }
}
