use stylist::css;
use yew::prelude::*;

use crate::style::colors::Color;
use crate::style::icons::IconMask;
use crate::style::themes::use_theme;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub mask: IconMask,
    pub fill: Color,
    pub width: Option<&'static str>,
    pub height: Option<&'static str>,
}

fn get_aspect_width(icon: IconMask, height: u16) -> u16 {
    let (x1, y1, x2, y2) = icon.viewbox();
    let w = x2 - x1;
    let h = y2 - y1;
    // w     ? = height * w / h
    // - = ------
    // h   height
    height * w / h
}

fn get_aspect_height(icon: IconMask, width: u16) -> u16 {
    let (w, h) = icon.vb_size();
    // w   width
    // - = -----
    // h     ? = width * h / w
    width * h / w
}

#[function_component(Icon)]
pub fn icon(props: &Props) -> Html {
    let theme = use_theme();

    let icon_style = css! {
        r#"
        & {
            width: ${width};
            height: ${height};
            display: inline-block;
            text-align: center;
            vertical-align: middle;
        }
        @media (min-width: 768px) {
            & {
                width: ${width_mobile};
                height: ${height_mobile};
            }
        }
        @media (min-width: 992px) {
            & {
                width: ${width_tablet};
                height: ${height_tablet};
            }
        }
        @media (min-width: 1200px) {
            & {
                width: ${width_desktop};
                height: ${height_desktop};
            }
        }
        "#,
        width = props.width.unwrap_or(theme.fs),
        height = props.height.unwrap_or(theme.fs),
        width_mobile = props.width.unwrap_or(theme.fsm),
        height_mobile = props.height.unwrap_or(theme.fsm),
        width_tablet = props.width.unwrap_or(theme.fst),
        height_tablet = props.height.unwrap_or(theme.fst),
        width_desktop = props.width.unwrap_or(theme.fsd),
        height_desktop = props.height.unwrap_or(theme.fsd),
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
