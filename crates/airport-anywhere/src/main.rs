use airport_anywhere::{
    services::AirportService,
    ui::pages::{Home, HomeProps},
};
use axum::{extract::Query, response::Html, routing::get, Router};
use serde::Deserialize;
use sky_tracer::protocol::airports::AirportResponse;
use tracing::{error, info, instrument};

#[derive(Debug, Deserialize)]
struct SearchParams {
    q: Option<String>,
}

#[instrument(skip(params))]
async fn render_page(Query(params): Query<SearchParams>) -> Html<String> {
    info!(?params, "Rendering page with search parameters");

    let airports: Vec<AirportResponse> = if let Some(query) = &params.q {
        info!(?query, "Searching for airports");

        // Try searching by code first, then by name
        let code_results = AirportService::search_by_code(query)
            .await
            .unwrap_or_default();

        if !code_results.is_empty() {
            info!("Found airports by code");
            code_results
        } else {
            info!("Searching airports by name");
            AirportService::search_by_name(query)
                .await
                .unwrap_or_default()
        }
    } else {
        info!("Listing all airports");
        match AirportService::get_all_airports().await {
            Ok(airports) => airports,
            Err(e) => {
                error!(error = %e, "Failed to load airports");
                vec![]
            }
        }
    };

    info!(airports_found = airports.len(), "Found airports");

    let renderer = yew::ServerRenderer::<Home>::with_props(move || HomeProps {
        airports,
        query: params.q,
    });

    let html = renderer.render().await;

    Html(format!(
        r#"<!DOCTYPE html>
        <html>
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1.0">
                <title>Airport Anywhere</title>
                <style>
                    {}
                </style>
            </head>
            <body>
                {}
            </body>
        </html>"#,
        include_str!("../assets/styles.css"),
        html
    ))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let _guard = init_tracing_opentelemetry::tracing_subscriber_ext::init_subscribers()?;

    // Get service configuration
    let service_port = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .unwrap_or(3000);

    let service_name =
        std::env::var("SERVICE_NAME").unwrap_or_else(|_| "airport-anywhere".to_string());

    info!("Starting {} service on port {}", service_name, service_port);

    let app = Router::new()
        .route("/", get(render_page))
        .merge(airport_anywhere::app());

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", service_port)).await?;
    info!("Server running on http://localhost:{}", service_port);
    info!(
        "API documentation available at http://localhost:{}/api/docs",
        service_port
    );

    // Run the server
    let server = axum::serve(listener, app);

    info!("Server started");

    // Run the server and handle shutdown
    if let Err(e) = server.await {
        tracing::error!("Server error: {}", e);
    }

    Ok(())
}
