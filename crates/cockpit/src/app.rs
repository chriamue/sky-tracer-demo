use crate::components::{FlightForm, FlightList, FlightMap, StatusPanel};
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div class="container">
            <header class="header">
                <h1>{"🎮 Sky Tracer Cockpit"}</h1>
                <p class="subtitle">{"Flight Management Dashboard"}</p>
            </header>

            <main class="main">
                <div class="dashboard">
                    <div class="dashboard-left">
                        <StatusPanel />
                        <FlightForm />
                    </div>
                    <div class="dashboard-right">
                        <FlightMap />
                        <FlightList />
                    </div>
                </div>
            </main>
        </div>
    }
}
