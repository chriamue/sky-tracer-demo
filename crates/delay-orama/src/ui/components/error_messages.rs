use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ErrorMessageProps {
    pub message: String,
}

#[function_component(ErrorMessage)]
pub fn error_message(props: &ErrorMessageProps) -> Html {
    html! {
        <div class="error-message">
            <h3>{"⚠️ Error"}</h3>
            <p>{&props.message}</p>
            <p>{"Please try again or contact support if the problem persists."}</p>
        </div>
    }
}
