use stylist::{style, yew::styled_component};
use yew::prelude::*;

use crate::style::themes::use_theme;

#[derive(Debug, PartialEq, Properties)]
pub struct Props {
    pub children: Children,
}

#[styled_component(ColorLink)]
pub fn view(props: &Props) -> Html {
    let theme = use_theme();

    let underline_style = style! {
        r#"
        & {
            height: 3px;
            width: 0%;
            transition: width 0.2s ease-out, background-color 0.5s;
            background-color: ${ac1};
        }
        "#,
        ac1 = theme.ac2,
    }
    .unwrap();

    let style = css! {
        r#"
        & {
            display: flex;
            flex-direction: column;
            justify-content: center;
            align-items: center;
            padding-bottom: 5px;
        }
        &:hover > .${uline} {
            width: 100%;
            background-color: ${ac2};
        }
        "#,
        uline = underline_style.get_class_name(),
        ac2 = theme.ac2,
    };

    html! {
        <div class={style}>
            <div>
                {props.children.clone()}
            </div>
            <div class={underline_style} />
        </div>
    }
}
