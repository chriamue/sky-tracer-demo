use axum::{routing::get, Router};
use tower_http::cors::{Any, CorsLayer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use airport_anywhere::openapi::ApiDoc;
use airport_anywhere::service::{list_airports, search_airports};

#[tokio::main]
async fn main() {
    // Build our application with routes
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
        .merge(SwaggerUi::new("/api/docs").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .merge(api_router);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running on http://localhost:3000");
    println!("API documentation available at http://localhost:3000/swagger-ui");
    axum::serve(listener, app).await.unwrap();
}
