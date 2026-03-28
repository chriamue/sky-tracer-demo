use crate::geo::SVG_WIDTH;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TitleProps {
    pub text: Option<String>,
}

#[function_component]
pub fn Title(props: &TitleProps) -> Html {
    match &props.text {
        Some(t) => {
            let tx = format!("{:.1}", SVG_WIDTH / 2.0);
            let text = t.clone();
            html! {
                <text x={tx} y="18" text-anchor="middle"
                      font-size="12" fill="#475569"
                      font-family="monospace" letter-spacing="2">
                    { text }
                </text>
            }
        }
        None => html! {},
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use yew::ServerRenderer;

    #[tokio::test]
    async fn renders_text_when_some() {
        let html = ServerRenderer::<Title>::with_props(|| TitleProps {
            text: Some("Sky Tracer".to_string()),
        })
        .render()
        .await;
        assert!(html.contains("Sky Tracer"));
        assert!(html.contains("<text"));
    }

    #[tokio::test]
    async fn renders_nothing_when_none() {
        let html = ServerRenderer::<Title>::with_props(|| TitleProps { text: None })
            .render()
            .await;
        assert!(!html.contains("<text"));
    }
}
