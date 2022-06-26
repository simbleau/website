use stylist::style;
use stylist::yew::styled_component;
use yew::prelude::*;

#[styled_component(Footer)]
pub fn footer() -> Html {
    let style = style!(
        r#"
            /* Anchor to bottom */
            position: absolute;
            bottom: 0;
            width: 100%;
            height: 50px;

            /* Additional styling */
            background-color: #dadada;
            padding: 5px;

            hr {
                width: 80%;
            }
        "#
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
