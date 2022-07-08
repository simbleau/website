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
    let icon_style = css! {
        & {
            width: var(--fs);
            height: var(--fs);
            display: inline-block;
            text-align: center;
            vertical-align: middle;
        }
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
