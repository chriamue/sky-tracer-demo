use delay_orama::{app::app, create_client, services::DelayService};
use std::env;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let _guard = init_tracing_opentelemetry::tracing_subscriber_ext::init_subscribers()?;

    // Get service configuration
    let service_port = env::var("PORT")
        .unwrap_or_else(|_| "3004".to_string())
        .parse()
        .unwrap_or(3004);

    let service_name = env::var("SERVICE_NAME").unwrap_or_else(|_| "delay-orama".to_string());

    let tower_babel_base_url = env::var("TOWER_BABEL_BASE_URL")
        .unwrap_or_else(|_| "http://tower-of-babel:3003".to_string());

    let airport_service_base_url = env::var("AIRPORT_SERVICE_BASE_URL")
        .unwrap_or_else(|_| "http://airport-anywhere:3000".to_string());

    info!("Starting {} service on port {}", service_name, service_port);
    info!(tower_babel_base_url = %tower_babel_base_url, "Configured Tower of Babel base URL");
    info!(airport_service_base_url = %airport_service_base_url, "Configured Airport Service base URL");

    let client = create_client();
    let delay_service = DelayService::new(client, tower_babel_base_url, airport_service_base_url);
    let app = app(delay_service);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", service_port)).await?;
    info!("Server running on http://localhost:{}", service_port);
    info!("Example usage: http://localhost:{}/FRA", service_port);

    let server = axum::serve(listener, app);

    info!("Server started successfully");

    if let Err(e) = server.await {
        tracing::error!("Server error: {}", e);
    }

    Ok(())
}
