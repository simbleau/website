use stylist::css;
use yew::prelude::*;

use crate::style::colors::Color;
use crate::style::icons::{Icon, IconMask};
use crate::style::themes::{use_theme, Theme, ThemeChoice};

const ICON_PADDING: &str = ".5em";

struct SwitcherColors {
    icon_color: Color,
    background_color: Color,
}

static MOUSE_INSIDE: fn(&Theme) -> SwitcherColors = |theme| SwitcherColors {
    icon_color: theme.fg2,
    background_color: theme.fg1.with_alpha(0.25),
};

static MOUSE_OUTSIDE: fn(&Theme) -> SwitcherColors = |theme| SwitcherColors {
    icon_color: theme.fg1,
    background_color: theme.fg1.with_alpha(0.10),
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

                width: calc(${width} + 2 * ${padding});
                height: calc(${height} + 2 * ${padding});
                border: 0;
                border-radius: 50%;
                text-align:center;

                transition: background-color 0.5s;
                background-color: ${bg};
            }
            @media (min-width: 768px) {
                & {
                    width: calc(${width_mobile} + 2 * ${padding});
                    height: calc(${height_mobile} + 2 * ${padding});
                }
            }
            @media (min-width: 992px) {
                & {
                    width: calc(${width_tablet} + 2 * ${padding});
                    height: calc(${height_tablet} + 2 * ${padding});
                }
            }
            @media (min-width: 1200px) {
                & {
                    width: calc(${width_desktop} + 2 * ${padding});
                    height: calc(${height_desktop} + 2 * ${padding});
                }
            }

            & i {
                transition: background 0s;
                pointer-events: none;
            }
        "#,
        width = theme.fs,
        height = theme.fs,
        width_mobile = theme.fsm,
        height_mobile = theme.fsm,
        width_tablet = theme.fst,
        height_tablet = theme.fst,
        width_desktop = theme.fsd,
        height_desktop = theme.fsd,
        padding = ICON_PADDING,
        bg = switcher_colors.background_color,
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
                    html!( <Icon mask={IconMask::Moon} fill={switcher_colors.icon_color} /> )
                }
                ThemeChoice::Dark => {
                    html!( <Icon mask={IconMask::Brightness} fill={switcher_colors.icon_color} /> )
                }
            }
        }
        </button>
    }
}
