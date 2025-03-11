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

#[function_component(HellAd)]
pub fn hell_ad(props: &HellAdProps) -> Html {
    let class = match props.position {
        HellAdPosition::Left => "hell-ad hell-ad-left",
        HellAdPosition::Right => "hell-ad hell-ad-right",
    };

    let prices = vec!["666", "999", "1666"];
    let price = use_state(|| prices[0]);

    let onclick = {
        let price = price.clone();
        let prices = prices.clone();
        Callback::from(move |_| {
            let next = prices
                .iter()
                .cycle()
                .skip_while(|&&p| p != *price)
                .nth(1)
                .unwrap();
            price.set(next);
        })
    };

    html! {
        <div class={class}>
            <div class="hell-content" {onclick}>
                <div class="hell-emojis">
                    {"👿"}
                    <div class="floating-emojis">
                        {"🔥"}{"😈"}{"🔥"}
                    </div>
                </div>
                <div class="hell-title">{"One-Way Trip to HELL"}</div>
                <div class="hell-slogan">{"Where delays are eternal!"}</div>
                <div class="hell-features">
                    <div>{"🌋 Infinite Layovers"}</div>
                    <div>{"♨️ Extra Hot Seats"}</div>
                    <div>{"👻 Ghost Crew"}</div>
                    <div>{"🏚️ Gates from 13-666"}</div>
                </div>
                <div class="hell-price">
                    <div class="price-tag">{"$"}{*price}</div>
                    <div class="soul-disclaimer">{"*Soul deposit required"}</div>
                </div>
                <div class="hell-button">
                    {"🔥 Book for Eternity 🔥"}
                </div>
            </div>
        </div>
    }
}
