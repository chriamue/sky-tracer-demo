use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SearchBoxProps {
    pub value: Option<String>,
    pub placeholder: String,
}

#[function_component(SearchBox)]
pub fn search_box(props: &SearchBoxProps) -> Html {
    html! {
        <form class="search-box" action="" method="get">
            <input
                type="text"
                name="q"
                value={props.value.clone().unwrap_or_default()}
                placeholder={props.placeholder.clone()}
                class="search-input"
            />
            <button type="submit" class="search-button">
                {"Search"}
            </button>
        </form>
    }
}
