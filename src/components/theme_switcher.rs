use stylist::css;
use yew::prelude::*;

use crate::style::colors::Color;
use crate::style::icons::{Icon, IconMask};
use crate::style::Theme;
use crate::style::{use_theme, ThemeChoice};

const ICON_SIZE: &str = "1.6em";
const ICON_PADDING: &str = ".5em";

struct SwitcherColors {
    icon: Color,
    circle: Color,
}

static MOUSE_INSIDE: fn(&Theme) -> SwitcherColors = |theme| SwitcherColors {
    icon: theme.fg2,
    circle: theme.fg1.with_alpha(0.25),
};

static MOUSE_OUTSIDE: fn(&Theme) -> SwitcherColors = |theme| SwitcherColors {
    icon: theme.fg1,
    circle: theme.fg1.with_alpha(0.10),
};

#[function_component(ThemeSwitcher)]
pub fn theme_switcher() -> Html {
    let theme = use_theme();
    let switcher_colors = use_state(|| MOUSE_OUTSIDE(&theme));

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
            }

            & i {
                transition: 0s;
                pointer-events: none;
            }
        "#,
        size = ICON_SIZE,
        padding = ICON_PADDING,
        bg = switcher_colors.circle,
    );

    let onmouseenter = {
        let theme = theme.clone();
        let hover_colors = switcher_colors.clone();
        move |_| hover_colors.set(MOUSE_INSIDE(&theme))
    };

    let onmouseleave = {
        let theme = theme.clone();
        let hover_colors = switcher_colors.clone();
        move |_| hover_colors.set(MOUSE_OUTSIDE(&theme))
    };

    let onclick = Callback::from({
        let theme = theme.clone();
        let switcher_colors = switcher_colors.clone();
        let other = match theme.kind() {
            ThemeChoice::Light => ThemeChoice::Dark,
            ThemeChoice::Dark => ThemeChoice::Light,
        };
        move |_| {
            theme.set(other);
            switcher_colors.set(MOUSE_INSIDE(other.theme()));
        }
    });

    html! {
        <button class={style} { onclick } {onmouseenter} {onmouseleave}>
        {
            match theme.kind() {
                ThemeChoice::Light => {
                    html!( <Icon mask={IconMask::Moon} fill={switcher_colors.icon} /> )
                }
                ThemeChoice::Dark => {
                    html!( <Icon mask={IconMask::Brightness} fill={switcher_colors.icon} /> )
                }
            }
        }
        </button>
    }
}
