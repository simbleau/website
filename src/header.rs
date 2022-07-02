use crate::navigation::Navigation;
use stylist::yew::styled_component;
use yew::prelude::*;

#[styled_component(Header)]
pub fn header() -> Html {
    let header_css = css!(
        r#"
            width: 100%;
            background-color: #dadada;
            padding-top: 5px;
            padding-bottom: 5px;
        "#
    );

    html! {
        <header class={ header_css }>
            <div id="nav-wrapper" align="center">
                <Navigation />
            </div>
        </header>
    }
}
