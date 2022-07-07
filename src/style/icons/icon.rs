use stylist::css;
use yew::prelude::*;

use crate::style::colors::Color;
use crate::style::icons::IconMask;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub mask: IconMask,
    pub fill: Color,
}

#[function_component(Icon)]
pub fn icon(props: &Props) -> Html {
    let class = css!(
        r#"
        & {
            -webkit-mask:url("${mask}");
            background: ${fill};
        }
        "#,
        mask = props.mask,
        fill = props.fill,
    );

    html! {
        <i class={classes!("ico", class)}></i>
    }
}
