use stylist::style;
use yew::prelude::*;

use crate::style::color::Color;
use crate::style::IconMask::*;

#[derive(PartialEq, Clone, Copy)]
pub enum IconMask {
    HalfFill,
}

impl IconMask {
    pub fn to_data(&self) -> &'static str {
        match self {
            HalfFill => {
                r#"data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24'%3E%3Cpath d='M12 0C6 0 0 6 0 12s5 12 12 12 12-5 12-12S19 0 12 0Zm0 4c5 0 8 3 8 8s-3 8-8 8V4Z'/%3E%3C/svg%3E"#
            }
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub mask: IconMask,
    pub fill: Color,
}

#[function_component(Icon)]
pub fn icon(props: &Props) -> Html {
    let class = style!(
        r#"
        & {
            -webkit-mask:url("${mask}");
            background: ${fill};
        }
        "#,
        mask = props.mask.to_data(),
        fill = props.fill,
    )
    .unwrap();

    html! {
        <i class={classes!("ico", class)}></i>
    }
}
