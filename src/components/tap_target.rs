use crate::{
    components::{Icon, IconMask},
    style::themes::ThemeChoice,
    util::lighten,
};
use hex_color::HexColor;
use stylist::css;
use themer::yew::use_theme;
use yew::prelude::*;

pub const SIZE: &str = "48px";
const FG_SIZE: &str = "24px";

#[derive(Properties, PartialEq)]
pub struct Props {
    pub mask: IconMask,
    #[prop_or_default]
    pub color: Option<HexColor>,
    #[prop_or_default]
    pub background: Option<HexColor>,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(TapTarget)]
pub fn view(props: &Props) -> Html {
    let theme = use_theme::<ThemeChoice>();

    // Coloring
    let bg = match props.background {
        Some(c) => c,
        None => theme.color.with_a(20), // 10% opacity
    };
    let fg = match props.color {
        Some(c) => c,
        None => theme.color,
    };
    let bg_hov = lighten(&bg, 0.8);
    let fg_hov = lighten(&fg, 1.2);

    let fg = format!("{fg:#}");
    let bg = format!("{bg:#}");
    let fg_hov = format!("{fg_hov:#}");
    let bg_hov = format!("{bg_hov:#}");

    let style = css! {
        display: inline-block;
        border: 0;
        border-radius: 50%;
        padding: 0;
        align-items: center;
        justify-content: center;
        cursor:pointer;

        width: ${SIZE};
        height: ${SIZE};

        transition: background-color 0.5s;
        background-color: ${bg};

        &:hover {
            background-color: ${bg_hov};
        }

        & > i {
            transition: width 0.25s, height 0.25s;
            background: ${fg};
            width: ${format!("calc({} * {})", FG_SIZE, IconMask::aspect_ratio(props.mask))};
            height: ${FG_SIZE};
        }

        &:hover > i {
            background: ${fg_hov};
            width: ${format!("calc({} * {})", FG_SIZE, IconMask::aspect_ratio(props.mask) * 1.25)};
            height: ${format!("calc({} * {})", FG_SIZE, 1.25)};
        }
    };

    html! {
        <button class={ classes!(style, props.class.clone()) }
            onclick={ props.onclick.clone() }
        >
        <Icon
            mask={ props.mask }
            class={ css!("pointer-events: none;") }
        />
        </button>
    }
}
