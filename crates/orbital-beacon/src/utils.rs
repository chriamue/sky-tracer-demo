pub fn get_path_prefix() -> String {
    std::env::var("PATH_PREFIX").unwrap_or_else(|_| "".to_string())
}
