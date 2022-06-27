use crate::common::components::nav::Navigation;
use stylist::style;
use stylist::yew::styled_component;
use yew::prelude::*;

#[styled_component(Header)]
pub fn header() -> Html {
    let header_css = css!(
        r#"
            width: 100%;
            background-color: #dadada;
            padding: 0px;
        "#
    );

    html! {
        <header class={ header_css }>
            <div align="center">
                <Navigation />
            </div>
            <hr class={css!("width: 100%;")} />
        </header>
    }
}
