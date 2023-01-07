use crate::components::{Icon, IconMask};
use crate::style::themes::BrandChoice;
use cssugar::prelude::*;
use stylist::css;
use themer::prelude::*;
use yew::prelude::*;

pub const SIZE: Length = Length::Px(48.0);
const FG_SIZE: Length = Length::Px(24.0);

#[derive(Properties, PartialEq)]
pub struct Props {
    pub mask: IconMask,
    #[prop_or_default]
    pub color: Option<Color>,
    #[prop_or_default]
    pub background: Option<Color>,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(TapTarget)]
pub fn view(props: &Props) -> Html {
    let theme = use_theme::<BrandChoice>();

    // Coloring
    let bg = match props.background {
        Some(c) => c,
        None => theme.color.alpha(0.10),
    };
    let fg = match props.color {
        Some(c) => c,
        None => theme.color,
    };

    let style = css!(
        r#"
        & {
            display: inline-block;
            border: 0;
            border-radius: 50%;
            padding: 0;
            align-items: center;
            justify-content: center;
            cursor:pointer;

            width: ${bgw};
            height: ${bgh};

            transition: background-color 0.5s;
            background-color: ${bg};
        }

        &:hover {
            background-color: ${bg_hover};
        }

        & > i {
            transition: width 0.5s, height 0.5s;
            background: ${fg};
            width: ${fgw};
            height: ${fgh};
        }

        &:hover > i {
            background: ${fg_hover};
            width: ${fgw_hover};
            height: ${fgh_hover};
        }

        "#,
        /* Coloring */
        bg = bg,
        bg_hover = bg.darken(0.2),
        fg = fg,
        fg_hover = fg.lighten(0.2),
        /* Sizing - Default */
        bgw = SIZE,
        bgh = SIZE,
        fgw = FG_SIZE * IconMask::aspect_ratio(props.mask),
        fgh = FG_SIZE,
        fgw_hover = FG_SIZE * IconMask::aspect_ratio(props.mask) * 1.25,
        fgh_hover = FG_SIZE * 1.25,
    );

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
