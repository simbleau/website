use crate::{
    components::{ExternalLink, Icon, IconMask},
    style::themes::ThemeChoice,
};
use js_sys::Date;
use stylist::yew::styled_component;
use themer::prelude::*;
use yew::prelude::*;

#[styled_component(Footer)]
pub fn footer() -> Html {
    let copyright_year = Date::new_0().get_full_year();
    let theme = use_theme::<ThemeChoice>();

    let css = css! {
        /* Sizing */
        padding-top: 5px;
        padding-bottom: 5px;
        width: 100%;

        /* Center footer line */
        display: flex;
        justify-content: center;
        align-items: center;
    };

    html! {
        <footer class={ css }>
            <div id="footer_wrap" align="center">
                <p>
                    { copyright_year }
                    { " " }
                    <Icon mask={IconMask::Copyright} fill={AttrValue::Static(theme.color)} />
                    { " Spencer C. Imbleau" }
                </p>
                {"made with "}
                <Icon mask={IconMask::Coffee} fill={theme.color} />
                {" and a "}
                <Icon mask={IconMask::Keyboard} fill={theme.color} />
                {" using " }
                <Icon mask={IconMask::Rust} fill={theme.color} />
                <br />
                <ExternalLink
                    icon={IconMask::GitHub}
                    to={AttrValue::Static("https://github.com/simbleau/website")}
                >
                    {"view source"}
                </ExternalLink>
            </div>
        </footer>
    }
}
