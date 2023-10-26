use crate::{
    components::{Icon, IconMask},
    style::themes::ThemeChoice,
};
use stylist::css;
use themer::yew::use_theme;
use yew::prelude::*;

pub const SIZE: &str = "48px";
const FG_SIZE: &str = "24px";

#[derive(Properties, PartialEq)]
pub struct Props {
    pub mask: IconMask,
    #[prop_or_default]
    pub color: Option<AttrValue>,
    #[prop_or_default]
    pub background: Option<AttrValue>,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(TapTarget)]
pub fn view(props: &Props) -> Html {
    let theme = use_theme::<ThemeChoice>();

    // Coloring
    let bg = match props.background.clone() {
        Some(c) => c,
        None => AttrValue::Rc(format!("darken({}, 90%)", theme.color).into()),
    };
    let fg = match props.color.clone() {
        Some(c) => c,
        None => AttrValue::Rc(theme.color.into()),
    };

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
            background-color: darken(${bg}, 20%);
        }

        & > i {
            transition: width 0.25s, height 0.25s;
            background: ${fg};
            width: ${format!("calc({} * {})", FG_SIZE, IconMask::aspect_ratio(props.mask))};
            height: ${FG_SIZE};
        }

        &:hover > i {
            background: lighten(${fg}, 20%);
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
