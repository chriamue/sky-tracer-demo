use yew::prelude::*;

#[function_component(SarcasticHeader)]
pub fn sarcastic_header() -> Html {
    html! {
        <header class="sarcastic-header">
            <div class="logo">{"✈️ Flightmare Airways"}</div>
            <div class="tagline">{"Where delays are our specialty!"}</div>
            <div class="awards">
                <span class="award" title="Most Creative Excuses 2024">{"🏆"}</span>
                <span class="award" title="Best Gate Changes">{"🚪"}</span>
                <span class="award" title="Outstanding Delay Times">{"⏰"}</span>
            </div>
            <div class="special-offer">
                {"🎉 Special Offer: Book now, fly whenever! 🎉"}
            </div>
        </header>
    }
}
