use crate::components::ThemeSwitcher;
use crate::navigation::Navigation;
use cssugar::prelude::*;
use stylist::yew::styled_component;
use yew::prelude::*;

pub const HEADER_PADDING: Length = Length::Px(5.0);
pub const HEADER_ITEM_PADDING: Length = Length::Px(5.0);

#[styled_component(Header)]
pub fn header() -> Html {
    let header_css = css! {
        padding-top: ${HEADER_PADDING};
        padding-bottom: ${HEADER_PADDING};
        width: 100%;
    };

    let list_css = css! {
        list-style-type: none;
        padding: 0;
        margin: 0;

        & li {
            padding: ${HEADER_ITEM_PADDING};
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
