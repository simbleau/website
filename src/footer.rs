use crate::style::themes::ThemeChoice;
use accessible_ui::prelude::*;
use cssugar::prelude::*;
use js_sys::Date;
use stylist::yew::styled_component;
use themer::prelude::*;
use yew::prelude::*;

pub const FOOTER_PADDING: Length = Length::Px(5.0);

#[styled_component(Footer)]
pub fn footer() -> Html {
    let copyright_year = Date::new_0().get_full_year();
    let theme = use_theme::<ThemeChoice>();

    let css = css!(
        r#"
            /* Sizing */
            padding-top: ${padding};
            padding-bottom: ${padding};
            width: 100%;

            /* Center footer line */
            display: flex;
            justify-content: center;
            align-items: center;
        "#,
        padding = FOOTER_PADDING,
    );

    html! {
        <footer class={ css }>
            <div id="footer_wrap" align="center">
                <p>
                    { copyright_year }
                    { " " }
                    <Icon mask={IconMask::Copyright} fill={theme.color} />
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
