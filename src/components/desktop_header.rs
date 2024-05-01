use crate::components::{Navigation, ThemeSwitcher};
use stylist::yew::styled_component;
use yew::prelude::*;

#[styled_component(DesktopHeader)]
pub fn view() -> Html {
    let header_css = css! {
        margin-bottom: 20px;
        width: 100%;
    };

    let list_css = css! {
        list-style-type: none;
        padding: 0;
        margin: 0;

        & li {
            padding-left: 10px;
            padding-right: 10px;
            display: inline-block;
        }
    };

    html! {
        <header class={ header_css }>
            <div id="nav-wrapper" align="center">
                <ul class={list_css}>
                    <li><ThemeSwitcher /></li>
                    <li><Navigation /></li>
                </ul>
            </div>
        </header>
    }
}
