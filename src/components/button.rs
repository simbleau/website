use stylist::css;
use yew::prelude::*;

use crate::style::use_theme;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub onclick: Callback<MouseEvent>,
    pub inner_text: fn() -> Html,
}

#[function_component(Button)]
pub fn button(props: &Props) -> Html {
    let theme = use_theme();

    let style = css!(
        r#"
            width: 2em;
            height: 2em;
            display:block;
            border: 0;
            border-radius: 50%;
            background-color: ${bg};
            color: ${fg};
            padding: 10px;
            cursor:pointer;
        "#,
        bg = theme.fg1,
        fg = theme.bg1,
    );

    html! {
        <button
            class={style}
            onclick={&props.onclick}
        >
        { (props.inner_text)() }
        </button>
    }
}
