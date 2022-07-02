use crate::navigation::Navigation;
use stylist::yew::styled_component;
use yew::prelude::*;

pub const HEADER_PADDING: &str = "5px";

#[styled_component(Header)]
pub fn header() -> Html {
    let header_css = css!(
        r#"
            width: 100%;
            background-color: #dadada;
            padding-top: ${header_padding};
            padding-bottom: ${header_padding};
        "#,
        header_padding = HEADER_PADDING,
    );

    html! {
        <header class={ header_css }>
            <div id="nav-wrapper" align="center">
                <Navigation />
            </div>
        </header>
    }
}
