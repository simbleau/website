use crate::style::themes::ThemeChoice;
use cssugar::prelude::*;
use stylist::css;
use themer::prelude::*;
use yew::prelude::*;

const BUTTON_BORDER_RADIUS: Length = Length::Px(5.0);
const BUTTON_PADDING: Length = Length::Px(10.0);

#[derive(Properties, PartialEq)]
pub struct Props {
    pub onclick: Callback<MouseEvent>,
    pub children: Children,
    pub class: Option<Classes>,
}

#[function_component(Button)]
pub fn view(props: &Props) -> Html {
    let theme = use_theme::<ThemeChoice>();

    let style = css!(
        cursor:pointer;
        padding: ${BUTTON_PADDING};
        text-align: center;

        border-radius: ${BUTTON_BORDER_RADIUS};
        border-width: 1px;
        border-style: solid;
        border-color: ${theme.color};

        color: ${theme.color};
        background: none;
        &:hover {
            background: ${theme.color.alpha(0.1)};
        }
    );

    html! {
        <button
            class={classes!(props.class.clone(), style)}
            onclick={&props.onclick}
        >
        { props.children.clone() }
        </button>
    }
}
