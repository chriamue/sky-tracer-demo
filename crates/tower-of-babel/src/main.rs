use std::env;
use tower_of_babel::{app::app, create_client, services::BabelService};
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let _guard = init_tracing_opentelemetry::tracing_subscriber_ext::init_subscribers()?;

    // Get service configuration
    let service_port = env::var("PORT")
        .unwrap_or_else(|_| "3003".to_string())
        .parse()
        .unwrap_or(3003);

    let service_name = env::var("SERVICE_NAME").unwrap_or_else(|_| "tower-of-babel".to_string());

    let flight_controller_base_url = env::var("FLIGHT_CONTROLLER_BASE_URL")
        .unwrap_or_else(|_| "http://flight-controller:3001".to_string());

    info!("Starting {} service on port {}", service_name, service_port);
    info!(flight_controller_base_url = %flight_controller_base_url, "Configured Flight Controller base URL");

    let client = create_client();
    let babel_service = BabelService::new(client, flight_controller_base_url);
    let app = app(babel_service);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", service_port)).await?;
    info!("Server running on http://localhost:{}", service_port);
    info!(
        "API documentation available at http://localhost:{}/api/docs",
        service_port
    );
    info!(
        "Example endpoint: http://localhost:{}/api/v1/babel/LAX",
        service_port
    );

    let server = axum::serve(listener, app);

    info!("Server started successfully");

    if let Err(e) = server.await {
        tracing::error!("Server error: {}", e);
    }

    Ok(())
}
