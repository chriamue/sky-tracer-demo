use rand::seq::SliceRandom;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct Destination {
    emoji: &'static str,
    name: &'static str,
    slogan: &'static str,
    price: u32,
}

const DESTINATIONS: &[Destination] = &[
    Destination {
        emoji: "🏖️",
        name: "Delayed Beach",
        slogan: "Eventually you'll get there!",
        price: 299,
    },
    Destination {
        emoji: "🗽",
        name: "New Queue City",
        slogan: "Stand in line like never before!",
        price: 499,
    },
    Destination {
        emoji: "🌋",
        name: "Chaos Island",
        slogan: "As unpredictable as our schedules!",
        price: 399,
    },
    Destination {
        emoji: "❄️",
        name: "Blizzard Bay",
        slogan: "Weather delays guaranteed!",
        price: 199,
    },
    Destination {
        emoji: "🎰",
        name: "Gate Roulette",
        slogan: "Bet on your gate number!",
        price: 599,
    },
    Destination {
        emoji: "🌧️",
        name: "Raincheck Rapids",
        slogan: "Maybe today, maybe tomorrow!",
        price: 149,
    },
];

#[function_component(AirlineAd)]
pub fn airline_ad() -> Html {
    let destination = use_state(|| {
        DESTINATIONS
            .choose(&mut rand::thread_rng())
            .unwrap()
            .clone()
    });

    let onclick = {
        let destination = destination.clone();
        Callback::from(move |_| {
            destination.set(
                DESTINATIONS
                    .choose(&mut rand::thread_rng())
                    .unwrap()
                    .clone(),
            );
        })
    };

    html! {
        <div class="airline-ad" {onclick}>
            <div class="ad-emoji">{destination.emoji}</div>
            <div class="ad-content">
                <h3>{destination.name}</h3>
                <p class="slogan">{destination.slogan}</p>
                <div class="price">
                    {"From "}
                    <span class="amount">{"$"}{destination.price}</span>
                    {"*"}
                </div>
                <div class="disclaimer">{"*Delays not included"}</div>
            </div>
        </div>
    }
}
