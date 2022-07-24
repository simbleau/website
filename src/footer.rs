use stylist::yew::styled_component;
use yew::prelude::*;

use crate::components::{Hyperlink, Url};
use crate::style::{
    icons::{Icon, IconMask},
    themes::use_theme,
};

pub const FOOTER_HEIGHT: &str = "150px";
pub const FOOTER_PADDING: &str = "5px";

#[styled_component(Footer)]
pub fn footer() -> Html {
    let theme = use_theme();

    let css = css!(
        r#"
            /* Anchor to bottom */
            position: absolute;
            bottom: 0;
            width: 100%;
            height: ${footer_height};

            /* Center footer line */
            display: flex;
            justify-content: center;
            align-items: center;

            /* Styling */
            padding-top: ${footer_padding};
            padding-bottom: ${footer_padding};
        "#,
        footer_height = FOOTER_HEIGHT,
        footer_padding = FOOTER_PADDING,
    );

    html! {
        <footer class={ css }>
            <div id="footer_wrap" align="center">
                <p>
                    { "2022 " }
                    <Icon mask={IconMask::Copyright} fill={theme.fg1} />
                    { " Spencer C. Imbleau" }
                </p>
                {"Made with "}
                <Icon mask={IconMask::Coffee} fill={theme.fg1} />
                {" and a "}
                <Icon mask={IconMask::Keyboard} fill={theme.fg1} />
                {" using " }
                <Icon mask={IconMask::Rust} fill={theme.fg1} />
                <br />
                <small>
                    <Hyperlink  icon={IconMask::PenToSquare}
                                domain={Url::External("https://github.com/simbleau/website")}
                                display={html!("Edit on GitHub")}
                    />
                </small>
            </div>
        </footer>
    }
}
