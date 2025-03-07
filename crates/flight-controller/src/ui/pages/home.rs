use crate::ui::components::FlightTable;
use sky_tracer::protocol::flights::FlightResponse;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct HomeProps {
    pub flights: Vec<FlightResponse>,
}

#[function_component(Home)]
pub fn home(props: &HomeProps) -> Html {
    html! {
        <div class="container">
            <header>
                <h1>{"✈️ Flight Controller"}</h1>
            </header>

            <main>
                <section class="search-section">
                    <form action="/" method="get">
                        <div class="form-group">
                            <label for="departure">{"From:"}</label>
                            <input type="text" id="departure" name="departure" />
                        </div>
                        <div class="form-group">
                            <label for="arrival">{"To:"}</label>
                            <input type="text" id="arrival" name="arrival" />
                        </div>
                        <div class="form-group">
                            <label for="date">{"Date:"}</label>
                            <input type="date" id="date" name="date" />
                        </div>
                        <button type="submit">{"Search Flights"}</button>
                    </form>
                </section>

                <section class="results-section">
                    <h2>{"Flight Schedule"}</h2>
                    if props.flights.is_empty() {
                        <p class="no-flights">{"No flights found"}</p>
                    } else {
                        <FlightTable flights={props.flights.clone()} />
                    }
                </section>
            </main>
        </div>
    }
}
