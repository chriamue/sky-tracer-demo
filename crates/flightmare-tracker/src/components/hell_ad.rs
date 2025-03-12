use rand::seq::SliceRandom;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct HellAdProps {
    pub position: HellAdPosition,
}

#[derive(PartialEq)]
pub enum HellAdPosition {
    Left,
    Right,
}

const SLOGANS: &[&str] = &[
    "First Class to Last Place!",
    "Where Comfort Comes to Die!",
    "No Refunds, Only Regrets!",
    "Like Heaven, But Different!",
    "Taking the 'Air' out of Airline!",
];

#[function_component(HellAd)]
pub fn hell_ad(props: &HellAdProps) -> Html {
    let class = match props.position {
        HellAdPosition::Left => "hell-ad hell-ad-left",
        HellAdPosition::Right => "hell-ad hell-ad-right",
    };

    let prices = vec!["404", "999"];
    let price = use_state(|| prices[0]);
    let slogan = use_state(|| SLOGANS[0]);

    let onclick = {
        let price = price.clone();
        let prices = prices.clone();
        let slogan = slogan.clone();
        Callback::from(move |_| {
            let next_price = prices
                .iter()
                .cycle()
                .skip_while(|&&p| p != *price)
                .nth(1)
                .unwrap();
            price.set(next_price);
            slogan.set(SLOGANS.choose(&mut rand::thread_rng()).unwrap());
        })
    };

    html! {
        <div class={class}>
            <div class="fun-hell-content" {onclick}>
                <div class="fun-title">
                    {"✈️ UnderWorld Airlines"}
                    <div class="fun-subtitle">{"(No Return Tickets!)"}</div>
                </div>

                <div class="mascot">
                    {"😈"}
                    <div class="mascot-accessories">
                        {"🎭"}{"🎪"}{"🎢"}
                    </div>
                </div>

                <div class="fun-slogan">{*slogan}</div>

                <div class="perks-list">
                    <div>{"🔥 Heated Economy Seats"}</div>
                    <div>{"🎮 Infernal Entertainment"}</div>
                    <div>{"🍖 Suspiciously Spicy Meals"}</div>
                    <div>{"🎵 Highway to Hell on Loop"}</div>
                </div>

                <div class="fun-price">
                    <div class="price-value">{"$"}{*price}</div>
                    <div class="price-terms">{"*Soul sold separately"}</div>
                </div>

                <button class="fun-button">
                    {"🎟️ Book if you dare! 🎪"}
                </button>

                <div class="fun-footer">
                    {"Rated #1 by Demons Daily!"}
                    <div class="tiny-text">{"(Our only competition is purgatory)"}</div>
                </div>
            </div>
        </div>
    }
}
