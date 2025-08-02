use sky_nexus::app;
use std::env;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let _guard = init_tracing_opentelemetry::tracing_subscriber_ext::init_subscribers()?;

    // Get service configuration
    let service_port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .unwrap_or(8080);

    let service_name = env::var("SERVICE_NAME").unwrap_or_else(|_| "sky-nexus".to_string());

    // Get service base URLs
    let airport_service_base_url = env::var("AIRPORT_SERVICE_BASE_URL")
        .unwrap_or_else(|_| "http://localhost:3000".to_string());

    let flight_service_base_url =
        env::var("FLIGHT_SERVICE_BASE_URL").unwrap_or_else(|_| "http://localhost:3001".to_string());

    let satellite_service_base_url = env::var("SATELLITE_SERVICE_BASE_URL")
        .unwrap_or_else(|_| "http://localhost:3002".to_string());

    info!("Starting {} service on port {}", service_name, service_port);
    info!(airport_service_base_url = %airport_service_base_url, "Configured Airport Service base URL");
    info!(flight_service_base_url = %flight_service_base_url, "Configured Flight Service base URL");
    info!(satellite_service_base_url = %satellite_service_base_url, "Configured Satellite Service base URL");

    let app = app::app();

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", service_port)).await?;
    info!("Sky Nexus listening on http://localhost:{}", service_port);
    info!(
        "API documentation available at http://localhost:{}/swagger-ui",
        service_port
    );

    let server = axum::serve(listener, app);

    info!("Server started successfully");

    if let Err(e) = server.await {
        tracing::error!("Server error: {}", e);
    }

    Ok(())
}
