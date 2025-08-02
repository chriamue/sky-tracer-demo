use flight_controller::app::app;
use sky_tracer::protocol::FLIGHTS_API_PATH;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let _guard = init_tracing_opentelemetry::tracing_subscriber_ext::init_subscribers()?;

    // Get service configuration
    let service_port = std::env::var("PORT")
        .unwrap_or_else(|_| "3001".to_string())
        .parse()
        .unwrap_or(3001);

    let service_name =
        std::env::var("SERVICE_NAME").unwrap_or_else(|_| "flight-controller".to_string());

    info!("Starting {} service on port {}", service_name, service_port);
    info!("Flight API available at: {}", FLIGHTS_API_PATH);

    let app = app();

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", service_port)).await?;
    info!("Server running on http://localhost:{}", service_port);
    info!(
        "API documentation available at http://localhost:{}/api/docs/",
        service_port
    );

    let server = axum::serve(listener, app);

    info!("Server started");

    // Run the server and handle shutdown
    if let Err(e) = server.await {
        tracing::error!("Server error: {}", e);
    }

    Ok(())
}
