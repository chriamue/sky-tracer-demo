use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    // This sets up tracing and OpenTelemetry (if configured via env)
    let _guard = init_tracing_opentelemetry::tracing_subscriber_ext::init_subscribers()?;

    info!("Starting Sky Nexus service");

    let app = sky_nexus::app();

    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    info!("Sky Nexus listening on http://localhost:8080");
    info!("API documentation available at http://localhost:8080/swagger-ui");

    // Run the server and handle shutdown
    if let Err(e) = axum::serve(listener, app).await {
        tracing::error!("Server error: {}", e);
    }

    Ok(())
}
