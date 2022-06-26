use crate::common::components::nav::Navigation;
use stylist::style;
use stylist::yew::styled_component;
use yew::prelude::*;

#[styled_component(Header)]
pub fn header() -> Html {
    let style = style!(
        r#"
            width: 100%;
            background-color: #dadada;
            padding: 5px;

            hr {
                width: 80%;
            }
        "#
    )
    .unwrap();

    html! {
        <header class={ style }>
            <div align="center">
                <Navigation />
            </div>
            <hr />
        </header>
    }
}
