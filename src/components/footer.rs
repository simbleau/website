use crate::{
    components::{ExternalLink, Icon, IconMask},
    hooks::use_theme,
};
use stylist::yew::styled_component;
use yew::prelude::*;

#[styled_component(Footer)]
pub fn view() -> Html {
    let theme = use_theme();

    let footer_css = css! {
        margin-top: 20px;
    };

    html! {
        <footer>
            <div class={footer_css}>
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
