use stylist::css;
use yew::prelude::*;

use crate::style::color::Color;
use crate::style::{use_theme, Icon, IconMask, ThemeChoice};

const ICON_SIZE: &str = "1.6em";
const ICON_PADDING: &str = ".5em";

#[function_component(ThemeSwitcher)]
pub fn theme_switcher() -> Html {
    let theme = use_theme();

    let style = css!(
        r#"
            & {
                display:block;
                cursor:pointer;

                border: 0;
                border-radius: 50%;
                width: calc(${size} + 2 * ${padding});
                height: calc(${size} + 2 * ${padding});
                text-align:center;

                transition: background-color 0.5s;
                background-color: ${bg};
                color: var(--fg1);
            }

            &:hover {
                background-color: ${bg_hover};
            }

            & i{
                font-size: ${size};
            }
        "#,
        size = ICON_SIZE,
        padding = ICON_PADDING,
        bg = theme.fg1.with_alpha(0.10),
        bg_hover = theme.fg1.with_alpha(0.25),
    );

    let other_theme = match theme.kind() {
        ThemeChoice::Light => ThemeChoice::Dark,
        ThemeChoice::Dark => ThemeChoice::Light,
    };
    let other_icon = match theme.kind() {
        ThemeChoice::Light => {
            html!( <Icon mask={IconMask::HalfFill} fill={theme.fg1} /> )
        }
        ThemeChoice::Dark => {
            html!( <Icon mask={IconMask::HalfFill} fill={theme.fg1} /> )
        }
    };
    let switch_theme = Callback::from(move |_| theme.set(other_theme));

    html! {
        <button class={style} onclick={ switch_theme }>
            { other_icon }
        </button>
    }
}
