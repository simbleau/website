use super::IconMask;
use crate::style::themes::ThemeChoice;
use hex_color::HexColor;
use stylist::css;
use themer::yew::use_theme;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub(crate) data_aui_id: Option<AttrValue>,
    #[prop_or_default]
    pub class: Classes,

    pub mask: IconMask,
    #[prop_or_default]
    pub fill: Option<HexColor>,
    #[prop_or_default]
    pub hover_fill: Option<HexColor>,
}

#[function_component(Icon)]
pub fn icon(props: &Props) -> Html {
    let theme = use_theme::<ThemeChoice>();

    let icon_style = css! {
        height: 0.8rem;
        width: ${
            format!("calc(0.8rem * {})", IconMask::aspect_ratio(props.mask))
        };
        display: inline-block;
        text-align: center;
        vertical-align: middle;
        text-decoration: none;
    };
    let mask_style = css! {
        r#"
            -webkit-mask:url("${mask}");
            background: ${bg};
            transition: background 0s;
        "#,
        mask = props.mask,
        bg = props.fill.unwrap_or(theme.color).display_rgba()
    };

    let mask_hover_style = props.hover_fill.map(|fill| {
        css! {
            &:hover {
                background: ${fill.display_rgba()};
            }
        }
    });

    html! {
        <i
            data-aui-id={props.data_aui_id.clone()}
            class={
                classes!(
                    icon_style,
                    mask_style,
                    mask_hover_style,
                    props.class.clone()
                )
            }
        />
    }
}
