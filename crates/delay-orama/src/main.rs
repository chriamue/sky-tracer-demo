use axum::{
    extract::{Path, State},
    response::Html,
    routing::get,
    Router,
};
use axum_tracing_opentelemetry::middleware::{OtelAxumLayer, OtelInResponseLayer};
use delay_orama::{
    create_client,
    ui::pages::{Home, HomeProps},
};
use reqwest_middleware::ClientWithMiddleware;
use sky_tracer::protocol::flights::FlightResponse;
use tower_http::cors::{Any, CorsLayer};
use tracing::{error, info, instrument, warn};

#[derive(Clone)]
struct AppState {
    tower_babel_url: String,
    client: ClientWithMiddleware,
}

#[instrument]
async fn render_page() -> Html<String> {
    let html = format!(
        r#"<!DOCTYPE html>
        <html>
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1.0">
                <title>Delay-O-Rama</title>
                <style>
                    {}
                </style>
            </head>
            <body>
                <div class="container">
                    <header>
                        <h1>⏰ Delay-O-Rama</h1>
                        <p>Enter an airport code in the URL (e.g., /delays/FRA) to see delays</p>
                    </header>
                </div>
            </body>
        </html>"#,
        include_str!("../assets/styles.css"),
    );

    Html(html)
}

#[instrument(skip(state))]
async fn render_airport_page(
    Path(airport_code): Path<String>,
    State(state): State<AppState>,
) -> Html<String> {
    info!(airport = %airport_code, "Fetching flights for airport");

    let url = format!("{}/api/babel/{}", state.tower_babel_url, airport_code);

    let response = state.client.get(&url).send().await;

    let flights = match response {
        Ok(res) => {
            if res.status().is_success() {
                match res.json::<Vec<FlightResponse>>().await {
                    Ok(flights) => {
                        info!(
                            airport = %airport_code,
                            flights_count = flights.len(),
                            "Retrieved flights successfully"
                        );
                        flights
                    }
                    Err(e) => {
                        error!(
                            airport = %airport_code,
                            error = %e,
                            "Failed to parse flights response"
                        );
                        Vec::new()
                    }
                }
            } else {
                warn!(
                    airport = %airport_code,
                    status = %res.status(),
                    "No flights found"
                );
                Vec::new()
            }
        }
        Err(e) => {
            error!(
                airport = %airport_code,
                error = %e,
                "Failed to fetch flights"
            );
            Vec::new()
        }
    };

    let renderer = yew::ServerRenderer::<Home>::with_props(move || HomeProps { flights });

    let html = renderer.render().await;

    Html(format!(
        r#"<!DOCTYPE html>
        <html>
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1.0">
                <title>Delay-O-Rama - {}</title>
                <style>
                    {}
                </style>
            </head>
            <body>
                {}
            </body>
        </html>"#,
        airport_code,
        include_str!("../assets/styles.css"),
        html
    ))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let _guard = init_tracing_opentelemetry::tracing_subscriber_ext::init_subscribers()?;

    let tower_babel_url = std::env::var("TOWER_BABEL_URL")
        .unwrap_or_else(|_| "http://tower-of-babel:3003".to_string());

    info!(url = %tower_babel_url, "Starting Delay-O-Rama service");

    let state = AppState {
        tower_babel_url,
        client: create_client(),
    };

    let app = Router::new()
        .route("/", get(render_page))
        .route("/{airport_code}", get(render_airport_page))
        .layer(OtelInResponseLayer::default())
        .layer(OtelAxumLayer::default())
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3004").await?;
    info!("Server running on http://localhost:3004");
    info!("Example usage: http://localhost:3004/FRA");

    let server = axum::serve(listener, app);

    info!("Server started");

    if let Err(e) = server.await {
        tracing::error!("Server error: {}", e);
    }

    Ok(())
}
