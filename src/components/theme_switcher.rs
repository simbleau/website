use stylist::css;
use yew::prelude::*;

use crate::style::{use_theme, ThemeChoice};

const ICON_SIZE: &str = "1.4em";
const ICON_PADDING: &str = ".4em";

#[function_component(ThemeSwitcher)]
pub fn theme_switcher() -> Html {
    let theme = use_theme();

    let style = css!(
        r#"
            display:block;
            cursor:pointer;

            border: 0;
            border-radius: 50%;
            width: calc(${size} + 2 * ${padding});
            height: calc(${size} + 2 * ${padding});
            text-align:center;

            background-color: ${bg};
            color: ${fg};

            i{
                font-size: ${size};
                text-decoration:none;
            }
        "#,
        size = ICON_SIZE,
        padding = ICON_PADDING,
        bg = theme.fg1.to_css(),
        fg = theme.bg1.to_css(),
    );

    let other_theme = match theme.kind() {
        ThemeChoice::Light => ThemeChoice::Dark,
        ThemeChoice::Dark => ThemeChoice::Light,
    };
    let other_icon: fn() -> Html = match theme.kind() {
        ThemeChoice::Light => || html!( <i class="i-moon" /> ),
        ThemeChoice::Dark => || html!( <i class="i-sun" /> ),
    };
    let switch_theme = Callback::from(move |_| {
        theme.set(other_theme)
        // TODO: Save setting in local storage
    });

    html! {
        <button class={style} onclick={ switch_theme }>
            { other_icon() }
        </button>
    }
}
