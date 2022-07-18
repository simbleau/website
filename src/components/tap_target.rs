use std::cell::RefCell;
use std::rc::Rc;

use stylist::css;
use web_sys::HtmlElement;
use yew::prelude::*;

use crate::style::colors::Color;
use crate::style::icons::{Icon, IconMask};
use crate::style::themes::{use_theme, ThemeChoice};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub mask: IconMask,
    #[prop_or_default]
    pub color: Option<Color>,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

#[function_component(TapTarget)]
pub fn view(props: &Props) -> Html {
    let theme = use_theme();

    let bg = theme.fg1.with_alpha(0.10);
    let hover_bg = theme.fg1.with_alpha(0.25);

    let fg = match props.color {
        Some(c) => c,
        None => theme.fg1,
    };

    let hover_fg = match theme.kind() {
        ThemeChoice::Dark => fg.lighten(0.3),
        ThemeChoice::Light => fg.darken(0.3),
    };

    let height = "24px";
    let width =
        format!("calc({} * {})", height, IconMask::aspect_ratio(props.mask));

    let hover_height = "28px";
    let hover_width = format!(
        "calc({} * {})",
        hover_height,
        IconMask::aspect_ratio(props.mask)
    );

    let style = css!(
        r#"
        & {
            display:block;
            cursor:pointer;

            width: 48px;
            height: 48px;
            border: 0;
            border-radius: 50%;
            text-align:center;

            transition: background-color 0.5s;
            background-color: ${bg};
        }

        &:hover {
            background-color: ${hover_bg};
        }

        & > i {
            transition: width 0.5s, height 0.5s;
            background: ${fg};
            width: ${width};
            height: ${height};
        }

        &:hover > i {
            background: ${hover_fg};
            width: ${hover_width};
            height: ${hover_height};
        }
        "#,
        bg = bg,
        hover_bg = hover_bg,
        fg = fg,
        hover_fg = hover_fg,
        width = width,
        height = height,
        hover_width = hover_width,
        hover_height = hover_height
    );

    let icon_style = css!(
        r#"
        & {
            pointer-events: none;
        }
        "#,
    );

    html! {
        <button class={style} onclick={ props.onclick.clone() } >
        <Icon
            mask={ props.mask }
            fs={"24px"}
            class={ icon_style }
        />
        </button>
    }
}
