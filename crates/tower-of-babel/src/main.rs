use axum::{extract::Path, extract::State, routing::get, Router};
use axum_tracing_opentelemetry::middleware::{OtelAxumLayer, OtelInResponseLayer};
use tower_http::cors::{Any, CorsLayer};
use tower_of_babel::{
    create_client,
    openapi::ApiDoc,
    service::{get_flight_position, list_flights_by_airport},
};
use tracing::info;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(Clone)]
struct AppState {
    flight_controller_url: String,
    client: reqwest_middleware::ClientWithMiddleware,
}

#[tracing::instrument(skip(state))]
async fn get_flights_by_airport(
    Path(airport_code): Path<String>,
    State(state): State<AppState>,
) -> impl axum::response::IntoResponse {
    list_flights_by_airport(
        state.flight_controller_url.clone(),
        airport_code,
        state.client.clone(),
    )
    .await
}

#[tracing::instrument(skip(state))]
async fn get_flight_position_handler(
    Path(flight_number): Path<String>,
    State(state): State<AppState>,
) -> impl axum::response::IntoResponse {
    get_flight_position(
        state.flight_controller_url.clone(),
        flight_number,
        state.client.clone(),
    )
    .await
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let _guard = init_tracing_opentelemetry::tracing_subscriber_ext::init_subscribers()?;

    let flight_controller_url = std::env::var("FLIGHT_CONTROLLER_URL")
        .unwrap_or_else(|_| "http://localhost:3001".to_string());

    info!(url = %flight_controller_url, "Starting Tower of Babel service");

    let state = AppState {
        flight_controller_url,
        client: create_client(),
    };

    let app = Router::new()
        .route("/api/babel/{airport_code}", get(get_flights_by_airport))
        .route(
            "/api/babel/{flight_number}/position",
            get(get_flight_position_handler),
        )
        .merge(SwaggerUi::new("/api/docs").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .layer(OtelInResponseLayer::default())
        .layer(OtelAxumLayer::default())
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3003").await?;
    info!("Server running on http://localhost:3003");
    info!("API documentation available at http://localhost:3003/api/docs");
    info!("Example endpoint: http://localhost:3003/api/babel/LAX");

    let server = axum::serve(listener, app);

    info!("Server started");

    if let Err(e) = server.await {
        tracing::error!("Server error: {}", e);
    }

    Ok(())
}
