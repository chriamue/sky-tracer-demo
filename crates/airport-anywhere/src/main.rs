use axum::{extract::Query, response::Html, routing::get, Json, Router};
use serde::Deserialize;
use tower_http::cors::{Any, CorsLayer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use airport_anywhere::{
    openapi::ApiDoc,
    service::{list_airports, search_airports},
    ui::pages::{Home, HomeProps},
};
use sky_tracer::protocol::airports::{AirportResponse, SearchAirportsRequest};

#[derive(Debug, Deserialize)]
struct SearchParams {
    q: Option<String>,
}

async fn render_page(Query(params): Query<SearchParams>) -> Html<String> {
    let airports: Vec<AirportResponse> = if let Some(query) = &params.q {
        // First try as IATA code
        let iata_request = SearchAirportsRequest {
            name: None,
            code: Some(query.clone()),
        };
        let Json(iata_response) = search_airports(Query(iata_request)).await;

        if !iata_response.airports.is_empty() {
            iata_response.airports
        } else {
            // Then try as ICAO code
            let icao_request = SearchAirportsRequest {
                name: None,
                code: Some(query.clone()),
            };
            let Json(icao_response) = search_airports(Query(icao_request)).await;

            if !icao_response.airports.is_empty() {
                icao_response.airports
            } else {
                // Finally, search by name
                let name_request = SearchAirportsRequest {
                    name: Some(query.clone()),
                    code: None,
                };
                let Json(name_response) = search_airports(Query(name_request)).await;
                name_response.airports
            }
        }
    } else {
        let Json(response) = list_airports().await;
        response.airports
    };

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
async fn main() {
    let api_router = Router::new()
        .route("/api/airports", get(list_airports))
        .route("/api/airports/search", get(search_airports))
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

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running on http://localhost:3000");
    println!("API documentation available at http://localhost:3000/swagger-ui");
    axum::serve(listener, app).await.unwrap();
}
