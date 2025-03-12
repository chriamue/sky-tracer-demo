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
use futures::future::join_all;
use reqwest_middleware::ClientWithMiddleware;
use sky_tracer::protocol::flights::FlightPositionResponse;
use sky_tracer::protocol::flights::FlightResponse;
use tower_http::cors::{Any, CorsLayer};
use tracing::{info, instrument};

#[derive(Clone)]
struct AppState {
    tower_babel_url: String,
    client: ClientWithMiddleware,
}

async fn fetch_flight_position(
    client: &ClientWithMiddleware,
    tower_babel_url: &str,
    flight_number: &str,
) -> Option<FlightPositionResponse> {
    let url = format!("{}/api/babel/{}/position", tower_babel_url, flight_number);

    match client.get(&url).send().await {
        Ok(res) => {
            if res.status().is_success() {
                res.json::<FlightPositionResponse>().await.ok()
            } else {
                None
            }
        }
        Err(_) => None,
    }
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

    let flights = match state.client.get(&url).send().await {
        Ok(res) => {
            if res.status().is_success() {
                res.json::<Vec<FlightResponse>>().await.unwrap_or_default()
            } else {
                Vec::new()
            }
        }
        Err(_) => Vec::new(),
    };

    // Fetch positions for all flights
    let position_futures = flights.iter().map(|flight| {
        fetch_flight_position(&state.client, &state.tower_babel_url, &flight.flight_number)
    });

    let positions = join_all(position_futures).await;

    // Combine flights with their positions
    let flights_with_positions: Vec<(FlightResponse, Option<FlightPositionResponse>)> =
        flights.into_iter().zip(positions).collect();

    // TODO: In a real application, you would fetch the airport's position from a service
    let airport_position = match airport_code.as_str() {
        "FRA" => Some((50.033333, 8.570556)),
        "CDG" => Some((49.012798, 2.55)),
        // Add more airports as needed
        _ => None,
    };

    let renderer = yew::ServerRenderer::<Home>::with_props(move || HomeProps {
        flights: flights_with_positions,
        airport_position,
    });

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
