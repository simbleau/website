use stylist::css;
use yew::prelude::*;

use crate::style::colors::Color;
use crate::style::icons::IconMask;
use crate::style::themes::use_theme;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub mask: IconMask,
    #[prop_or_default]
    pub fill: Option<Color>,
    #[prop_or_default]
    pub hover_fill: Option<Color>,
    #[prop_or_default]
    pub fs: Option<&'static str>,
    #[prop_or_default]
    pub class: Classes,
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
        width = format!("calc({} * {})", props.fs.unwrap_or(theme.fs),
            IconMask::aspect_ratio(props.mask)).as_str(),
        height = props.fs.unwrap_or(theme.fs),
        width_mobile = format!("calc({} * {})", props.fs.unwrap_or(theme.fsm),
            IconMask::aspect_ratio(props.mask)).as_str(),
        height_mobile = props.fs.unwrap_or(theme.fsm),
        width_tablet = format!("calc({} * {})", props.fs.unwrap_or(theme.fst),
            IconMask::aspect_ratio(props.mask)).as_str(),
        height_tablet = props.fs.unwrap_or(theme.fst),
        width_desktop = format!("calc({} * {})", props.fs.unwrap_or(theme.fsd),
            IconMask::aspect_ratio(props.mask)).as_str(),
        height_desktop = props.fs.unwrap_or(theme.fsd),
    };

    let mask_style = css!(
        r#"
        & {
            -webkit-mask:url("${mask}");
            background: ${fill};
            transition: background 0s;
        }
        "#,
        mask = props.mask,
        fill = props.fill.unwrap_or(theme.fg1),
    );

    let mask_hover_style = props.hover_fill.map(|fill| {
        css!(
            r#"
            &:hover {
                background: ${hover_fill};
            }
            "#,
            hover_fill = fill,
        )
    });

    html! {
        <i class={classes!(icon_style, mask_style, mask_hover_style, props.class.clone())}></i>
    }
}
