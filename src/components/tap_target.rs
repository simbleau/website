use crate::{
    components::{Icon, IconMask},
    hooks::use_theme,
};
use hex_color::HexColor;
use stylist::{css, yew::use_media_query};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub mask: IconMask,
    pub aria_label: AttrValue,
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
    let theme = use_theme();

    let is_mobile = use_media_query("(max-width: 992px)");
    let (size, fg_size) = if is_mobile {
        ("48px", "24px")
    } else {
        ("64px", "32px")
    };

    // Coloring
    let bg = match props.background {
        Some(c) => c,
        None => theme.color.with_a(20), // 10% opacity
    };
    let fg = match props.color {
        Some(c) => c,
        None => theme.color,
    };
    let bg_hov = bg.scale(0.8).display_rgba();
    let fg_hov = fg.scale(1.2).display_rgba();

    let style = css! {
        display: inline-block;
        border: 0;
        border-radius: 50%;
        padding: 0;
        align-items: center;
        justify-content: center;
        cursor:pointer;

        width: ${size};
        height: ${size};

        transition: background-color 0.5s;
        background-color: ${bg.display_rgba()};

        &:hover {
            background-color: ${bg_hov};
        }

        & > i {
            transition: width 0.25s, height 0.25s;
            background: ${fg.display_rgba()};
            width: ${format!("calc({} * {})", fg_size, IconMask::aspect_ratio(props.mask))};
            height: ${fg_size};
        }

        &:hover > i {
            background: ${fg_hov};
            width: ${format!("calc({} * {})", fg_size, IconMask::aspect_ratio(props.mask) * 1.25)};
            height: ${format!("calc({} * {})", fg_size, 1.25)};
        }
    };

    html! {
        <button
            class={ classes!(style, props.class.clone()) }
            onclick={ props.onclick.clone() }
            aria-label={ props.aria_label.clone() }
        >
        <Icon
            mask={ props.mask }
            class={ css!("pointer-events: none;") }
        />
        </button>
    }
}
