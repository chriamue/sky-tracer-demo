use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct NotFoundProps {
    pub airport_code: String,
}

#[function_component(NotFound)]
pub fn not_found(props: &NotFoundProps) -> Html {
    html! {
        <div class="error-message">
            <h3>{"✈️ Airport Not Found"}</h3>
            <p>{format!("Airport '{}' was not found in our database.", props.airport_code)}</p>
            <p>{"Please check the airport code and try again."}</p>
            <div class="suggestions">
                <p><strong>{"Try these popular airports:"}</strong></p>
                <a href="/delays/FRA">{"Frankfurt (FRA)"}</a> {" | "}
                <a href="/delays/CDG">{"Paris CDG"}</a> {" | "}
                <a href="/delays/LAX">{"Los Angeles (LAX)"}</a> {" | "}
                <a href="/delays/JFK">{"New York JFK"}</a>
            </div>
        </div>
    }
}
