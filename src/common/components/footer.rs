use stylist::yew::styled_component;
use yew::prelude::*;

pub const FOOTER_HEIGHT: &str = "100px";

#[styled_component(Footer)]
pub fn footer() -> Html {
    let footer_class = css!(
        r#"
            /* Anchor to bottom */
            position: absolute;
            bottom: 0;
            width: 100%;
            height: ${footer_height};
            overflow: hidden;

            /* Additional styling */
            background-color: #dadada;
        "#,
        footer_height = FOOTER_HEIGHT
    );

    html! {
        <footer class={ footer_class }>
            <hr class={css!("width: 100%;")} />
            <div align="center">
                <p>{ "Made with coffee and Rust " }</p>
            </div>
        </footer>
    }
}
