use stylist::{style, yew::styled_component};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
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
            padding: 0;
            margin: 0;
            display: flex;
            justify-content: center;
            align-items: center;
            min-height: 100%;
            flex-direction: column;
            background-color: rgb(20, 20, 20);

            font-family: sans-serif;
            color: rgb(200, 200, 200);
        "#
    )
    .unwrap();

    html! {
        <div align="center" class={ container_style }>
            <img src = "static/construction_image.svg"
                alt="Under Construction Image"
                width=144px
                height=144px
            />
            <h1>{ &props.title }</h1>
            <h3>{ &props.message }</h3>
            if let Some(end) = &props.end {
                <p>{ "Expected completion: " } { end }</p>
            }
        </div>
    }
}
