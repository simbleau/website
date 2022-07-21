use stylist::yew::styled_component;
use yew::prelude::*;

use crate::components::ThemeSwitcher;
use crate::navigation::Navigation;
use crate::style::themes::use_theme;

pub const HEADER_PADDING: &str = "5px";

#[styled_component(Header)]
pub fn header() -> Html {
    let theme = use_theme();

    let header_css = css!(
        r#"
            width: 100%;
            padding-top: ${header_padding};
            padding-bottom: ${header_padding};

            ul {
                list-style-type: none;
                padding: 0;
                margin: 0;
            }

            li {
                padding: ${header_padding};
                display: inline-block;
            }
        "#,
        header_padding = HEADER_PADDING,
    );

    html! {
        <header class={ header_css }>
            <div id="nav-wrapper" align="center">
                <ul>
                    <li><Navigation /></li>
                    <li><ThemeSwitcher /></li>
                </ul>
            </div>
        </header>
    }
}
