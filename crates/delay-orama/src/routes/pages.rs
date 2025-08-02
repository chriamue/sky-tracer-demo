use crate::{
    services::DelayService,
    ui::pages::{Home, HomeProps},
};
use axum::{
    extract::{Path, State},
    response::Html,
};
use tracing::{info, instrument};

#[instrument]
pub async fn render_home_page() -> Html<String> {
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
                        <p>Supported airports: FRA, CDG, LAX, JFK, and many more!</p>
                        <div class="example-links">
                            <a href="/delays/FRA">Frankfurt (FRA)</a> |
                            <a href="/delays/CDG">Paris CDG</a> |
                            <a href="/delays/LAX">Los Angeles (LAX)</a> |
                            <a href="/delays/JFK">New York JFK</a>
                        </div>
                    </header>
                </div>
            </body>
        </html>"#,
        include_str!("../../assets/styles.css"),
    );

    Html(html)
}

#[instrument(skip(service), fields(airport_code = %airport_code))]
pub async fn render_airport_delays(
    Path(airport_code): Path<String>,
    State(service): State<DelayService>,
) -> Html<String> {
    info!("Rendering delays page for airport: {}", airport_code);

    let (flights_with_positions, airport_position, error_message) =
        service.get_airport_delays_with_errors(&airport_code).await;

    // Clone airport_code before moving it into the closure
    let airport_code_for_title = airport_code.clone();
    let airport_code_for_props = airport_code.clone();

    let renderer = yew::ServerRenderer::<Home>::with_props(move || HomeProps {
        flights: flights_with_positions,
        airport_position,
        airport_code: Some(airport_code_for_props),
        error_message,
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
        airport_code_for_title,
        include_str!("../../assets/styles.css"),
        html
    ))
}
