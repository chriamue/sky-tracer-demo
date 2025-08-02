pub fn get_path_prefix() -> String {
    std::env::var("PATH_PREFIX").unwrap_or_else(|_| "".to_string())
}

/// Get the base URL for this service (used for external references)
pub fn get_service_base_url() -> String {
    let path_prefix = get_path_prefix();
    if path_prefix.is_empty() {
        "/".to_string()
    } else {
        path_prefix
    }
}
