use crate::style::themes::BrandChoice;
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
    let theme = use_theme::<BrandChoice>();

    let style = css!(
        & {
            border-radius: ${BUTTON_BORDER_RADIUS};
            border: 1px solid ${theme.color};
            background: none;

            text-align: center;
            color: ${theme.color};
            padding: ${BUTTON_PADDING};
            cursor:pointer;
        }
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
