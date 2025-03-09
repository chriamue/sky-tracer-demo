use airport_anywhere::{
    openapi::ApiDoc,
    service::{list_airports, search_airports},
    ui::pages::{Home, HomeProps},
};
use axum::{extract::Query, response::Html, routing::get, Json, Router};
use axum_tracing_opentelemetry::middleware::{OtelAxumLayer, OtelInResponseLayer};
use serde::Deserialize;
use sky_tracer::protocol::airports::{AirportResponse, SearchAirportsRequest};
use tower_http::cors::{Any, CorsLayer};
use tracing::{info, instrument};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(Debug, Deserialize)]
struct SearchParams {
    q: Option<String>,
}

#[instrument(skip(params))]
async fn render_page(Query(params): Query<SearchParams>) -> Html<String> {
    info!(?params, "Rendering page with search parameters");

    let airports: Vec<AirportResponse> = if let Some(query) = &params.q {
        info!(?query, "Searching for airports");
        // First try as IATA code
        let iata_request = SearchAirportsRequest {
            name: None,
            code: Some(query.clone()),
        };
        let Json(iata_response) = search_airports(Query(iata_request)).await;

        if !iata_response.airports.is_empty() {
            info!("Found airports by IATA code");
            iata_response.airports
        } else {
            // Then try as ICAO code
            let icao_request = SearchAirportsRequest {
                name: None,
                code: Some(query.clone()),
            };
            let Json(icao_response) = search_airports(Query(icao_request)).await;

            if !icao_response.airports.is_empty() {
                info!("Found airports by ICAO code");
                icao_response.airports
            } else {
                // Finally, search by name
                info!("Searching airports by name");
                let name_request = SearchAirportsRequest {
                    name: Some(query.clone()),
                    code: None,
                };
                let Json(name_response) = search_airports(Query(name_request)).await;
                name_response.airports
            }
        }
    } else {
        info!("Listing all airports");
        let Json(response) = list_airports().await;
        response.airports
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

    info!("Starting Airport Anywhere service");

    let api_router = Router::new()
        .route("/api/airports", get(list_airports))
        .route("/api/airports/search", get(search_airports))
        .layer(OtelInResponseLayer::default())
        .layer(OtelAxumLayer::default())
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        );

    let app = Router::new()
        .route("/", get(render_page))
        .merge(SwaggerUi::new("/api/docs").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .merge(api_router);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    info!("Server running on http://localhost:3000");
    info!("API documentation available at http://localhost:3000/api/docs");

    // Run the server
    let server = axum::serve(listener, app);

    info!("Server started");

    // Run the server and handle shutdown
    if let Err(e) = server.await {
        tracing::error!("Server error: {}", e);
    }

    Ok(())
}
