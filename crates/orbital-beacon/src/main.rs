use orbital_beacon::{app::app, services::SatelliteService, utils::get_path_prefix};
use std::env;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let _guard = init_tracing_opentelemetry::tracing_subscriber_ext::init_subscribers()?;

    // Get service configuration
    let service_port = std::env::var("PORT")
        .unwrap_or_else(|_| "3002".to_string())
        .parse()
        .unwrap_or(3002);

    let service_name =
        std::env::var("SERVICE_NAME").unwrap_or_else(|_| "orbital-beacon".to_string());

    // Fixed: Use AIRPORTS_SERVICE_BASE_URL to match other services
    let airport_service_url = env::var("AIRPORTS_SERVICE_BASE_URL")
        .unwrap_or_else(|_| "http://localhost:3000".to_string());

    info!("Starting {} service on port {}", service_name, service_port);
    info!(airport_service_url = %airport_service_url, "Configured airport service");

    let satellite_service = SatelliteService::new(airport_service_url);
    let app = app(satellite_service);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", service_port)).await?;
    let path_prefix = get_path_prefix();

    info!(
        path_prefix = %path_prefix,
        port = service_port,
        "Server starting"
    );
    info!(
        "API documentation available at http://localhost:{}/api/docs",
        service_port
    );

    let server = axum::serve(listener, app);

    info!("Server started");

    if let Err(e) = server.await {
        tracing::error!("Server error: {}", e);
    }

    Ok(())
}
