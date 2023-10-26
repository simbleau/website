use crate::{components::ThemeSwitcher, navigation::Navigation};
use stylist::yew::styled_component;
use yew::prelude::*;

#[styled_component(Header)]
pub fn header() -> Html {
    let header_css = css! {
        padding-top: 5px;
        padding-bottom: 5px;
        width: 100%;
    };

    let list_css = css! {
        list-style-type: none;
        padding: 0;
        margin: 0;

        & li {
            padding-left: 5px;
            padding-right: 5px;
            display: inline-block;
        }
    };

    html! {
        <header class={ header_css }>
            <div id="nav-wrapper" align="center">
                <ul class={list_css}>
                    <li><Navigation /></li>
                    <li><ThemeSwitcher /></li>
                </ul>
            </div>
        </header>
    }
}
