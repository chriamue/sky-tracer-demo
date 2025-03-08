use axum::extract::State;
use axum::{
    response::Html,
    routing::{get, post, put},
    Router,
};
use orbital_beacon::{
    openapi::ApiDoc,
    satellite_service::SatelliteService,
    service,
    ui::pages::{Home, HomeProps},
};
use tower_http::cors::{Any, CorsLayer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

async fn render_page(State(service): axum::extract::State<SatelliteService>) -> Html<String> {
    let satellites = service.list_satellites().await;
    let satellites = satellites
        .into_iter()
        .map(|s| sky_tracer::protocol::protocol::SatelliteResponse {
            id: s.id,
            name: s.name,
            status: s.status,
        })
        .collect();

    let renderer = yew::ServerRenderer::<Home>::with_props(move || HomeProps { satellites });

    let html = renderer.render().await;

    Html(format!(
        r#"<!DOCTYPE html>
        <html>
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1.0">
                <title>Orbital Beacon</title>
                <style>
                    {}
                </style>
            </head>
            <body>
                {}
            </body>
        </html>"#,
        include_str!("../styles.css"),
        html
    ))
}

#[tokio::main]
async fn main() {
    let satellite_service = SatelliteService::new();

    let app = Router::new()
        .route("/", get(render_page))
        .route("/api/satellites", post(service::create_satellite))
        .route(
            "/api/satellites/{id}/status",
            put(service::update_satellite_status),
        )
        .route("/api/satellites", get(service::list_satellites))
        .route("/api/position", post(service::calculate_position))
        .merge(SwaggerUi::new("/api/docs").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .with_state(satellite_service);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3002").await.unwrap();
    println!("Orbital Beacon running on http://localhost:3002");
    println!("API documentation available at http://localhost:3002/api/docs");
    axum::serve(listener, app).await.unwrap();
}
