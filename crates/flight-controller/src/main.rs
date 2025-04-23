use flight_controller::app::app;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let _guard = init_tracing_opentelemetry::tracing_subscriber_ext::init_subscribers()?;

    let app = app();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await?;
    info!("Server running on http://localhost:3001");
    info!("API documentation available at http://localhost:3001/api/docs");

    let server = axum::serve(listener, app);

    info!("Server started");

    // Run the server and handle shutdown
    if let Err(e) = server.await {
        tracing::error!("Server error: {}", e);
    }

    Ok(())
}
