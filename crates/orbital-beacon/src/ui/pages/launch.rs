use crate::ui::components::LaunchSatellite;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct LaunchProps {
    pub flash_message: Option<String>,
}

#[function_component(Launch)]
pub fn launch(props: &LaunchProps) -> Html {
    html! {
        <div class="container">
            <header>
                <h1>{"🚀 Launch Satellite"}</h1>
            </header>

            {if let Some(message) = &props.flash_message {
                html! {
                    <div class="flash-message">
                        {message}
                    </div>
                }
            } else {
                html! {}
            }}

            <main>
                <LaunchSatellite />
                <a href={format!("{}/", crate::utils::get_path_prefix())} class="back-link">{"Back to Home"}</a>
            </main>
        </div>
    }
}
