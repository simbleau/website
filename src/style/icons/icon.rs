use stylist::css;
use yew::prelude::*;

use crate::style::colors::Color;
use crate::style::icons::IconMask;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub mask: IconMask,
    pub fill: Color,
    pub width: Option<&'static str>,
    pub height: Option<&'static str>,
}

#[function_component(Icon)]
pub fn icon(props: &Props) -> Html {
    let icon_style = css! {
        r#"
        & {
            width: ${width};
            height: ${height};
            display: inline-block;
            text-align: center;
            vertical-align: middle;
        }
        "#,
        width = props.width.unwrap_or("1em"),
        height = props.height.unwrap_or("1em"),
    };

    let mask_style = css!(
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
        <i class={classes!(icon_style, mask_style)}></i>
    }
}
