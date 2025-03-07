use crate::ui::components::{AirportCard, SearchBox};
use sky_tracer::protocol::airports::AirportResponse;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct HomeProps {
    pub airports: Vec<AirportResponse>,
    pub query: Option<String>,
}

#[function_component(Home)]
pub fn home(props: &HomeProps) -> Html {
    html! {
        <div class="container">
            <header class="header">
                <h1>{"✈️ Airport Anywhere"}</h1>
                <p class="subtitle">{"Find airports around the world"}</p>
            </header>

            <main class="main">
                <section class="search-section">
                    <SearchBox
                        value={props.query.clone()}
                        placeholder="Search airports by name..."
                    />
                </section>

                <section class="results-section">
                    if props.airports.is_empty() {
                        <div class="no-results">
                            {"No airports found"}
                        </div>
                    } else {
                        <div class="airport-grid">
                            {for props.airports.iter().map(|airport| {
                                html! {
                                    <AirportCard airport={airport.clone()} />
                                }
                            })}
                        </div>
                    }
                </section>
            </main>
        </div>
    }
}
