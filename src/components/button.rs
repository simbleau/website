use crate::style::themes::BrandChoice;
use stylist::css;
use themer::prelude::*;
use yew::prelude::*;

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
        r#"
        & {
            border: 0;
            border-radius: 5px;

            border: 1px solid ${fg};
            background: none;

            text-align: center;
            color: ${fg};
            padding: 10px;
            cursor:pointer;
        }

        &:hover {
            background: ${bg};
        }
        "#,
        fg = theme.color,
        bg = theme.color.alpha(0.1),
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
