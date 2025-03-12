use crate::components::{AirlineAd, AsyncFlightList, FlightList, SarcasticHeader};
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <>
            <div class="container">
                <SarcasticHeader />
                <main>
                <FlightList />
                <AsyncFlightList />
                </main>
                <div class="ads-container">
                    <AirlineAd />
                </div>
            </div>
        </>
    }
}
