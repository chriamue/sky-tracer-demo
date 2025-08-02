use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct UpdateStatusProps {
    pub flash_message: Option<String>,
}

#[function_component(UpdateStatus)]
pub fn update_status(props: &UpdateStatusProps) -> Html {
    let path_prefix = crate::utils::get_path_prefix();

    html! {
        <div class="container">
            <header>
                <h1>{"🛰️ Update Satellite Status"}</h1>
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
                <p>{"Select a satellite to update its status:"}</p>
                <a href={path_prefix} class="back-link">{"Back to Home"}</a>
            </main>
        </div>
    }
}
