use crate::grund::Grund;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct GrundDisplayProps {
    pub grund: Option<Grund>,
}

#[function_component(GrundDisplay)]
pub fn grund_display(props: &GrundDisplayProps) -> Html {
    let (status_class, message) = match &props.grund {
        Some(grund) => ("delay", grund.to_string()),
        None => ("on-time", "On Time ✓".to_string()),
    };

    html! {
        <div class={classes!("grund-display", status_class)}>
            <div class="status-icon">
                {
                    if props.grund.is_some() {
                        "⚠️"
                    } else {
                        "✈️"
                    }
                }
            </div>
            <div class="status-message">
                {message}
            </div>
        </div>
    }
}
