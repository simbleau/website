use stylist::style;
use stylist::yew::styled_component;
use yew::prelude::*;

pub const FOOTER_HEIGHT: &str = "100px";

#[styled_component(Footer)]
pub fn footer() -> Html {
    let style = style!(
        r#"
            /* Anchor to bottom */
            position: absolute;
            bottom: 0;
            width: 100%;
            height: ${footer_height};
            overflow: hidden;

            /* Additional styling */
            background-color: #dadada;

            hr {
                width: 80%;
            }
        "#,
        footer_height = FOOTER_HEIGHT
    )
    .unwrap();

    html! {
        <footer class={ style }>
            <hr />
            <div align="center">
                <p>{ "Made with coffee and Rust " }</p>
            </div>
        </footer>
    }
}
