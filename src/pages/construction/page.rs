use stylist::{style, yew::styled_component};
use yew::prelude::*;

#[derive(Properties, PartialEq, Eq)]
pub struct ConstructionMessage {
    #[prop_or("Under Construction!".to_string())]
    pub title: String,
    #[prop_or("Pardon my dust.".to_string())]
    pub message: String,
    #[prop_or_default]
    pub end: Option<String>,
}

#[styled_component(ConstructionPage)]
pub fn construction_page(props: &ConstructionMessage) -> Html {
    let container_style = style!(
        r#"
            padding: 10px;
            margin: 0;

            display: flex;
            justify-content: center;
            align-items: center;
            flex-direction: column;
            min-height: 100%;
        "#
    )
    .unwrap();

    html! {
        <div class={ container_style }>
            <img src = "static/construction_image.svg"
                alt="Under Construction Image"
                width=144px
                height=144px
            />
            <h1>{ &props.title }</h1>
            <h2>{ &props.message }</h2>
            if let Some(end) = &props.end {
                <p>{ "Expected completion: " } { end }</p>
            }
        </div>
    }
}
