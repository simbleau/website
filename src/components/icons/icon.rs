use crate::style::themes::ThemeChoice;

use super::IconMask;
use gloo_utils::window;
use hex_color::HexColor;
use stylist::css;
use themer::yew::use_theme;
use web_sys::HtmlElement;
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

    let height = use_state(|| "1em".to_string());
    let width = use_state(|| "1em".to_string());
    let icon_ref = use_node_ref();
    {
        let mask = props.mask;
        let height = height.clone();
        let width = width.clone();
        let icon_ref = icon_ref.clone();
        use_effect_with(icon_ref, move |icon_ref| {
            if let Some(element) = icon_ref.cast::<HtmlElement>() {
                let computed_height = window()
                    .get_computed_style(&element)
                    .unwrap()
                    .unwrap()
                    .get_property_value("font-size")
                    .unwrap();
                let computed_width = format!(
                    "calc({} * {})",
                    computed_height,
                    IconMask::aspect_ratio(mask)
                );
                height.set(computed_height);
                width.set(computed_width);
            }

            move || {}
        });
    }

    let icon_style = css! {
        width: ${*width};
        height: ${*height};
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
        bg = props.fill.unwrap_or(theme.color)
    };

    let mask_hover_style = props.hover_fill.map(|fill| {
        css! {
            &:hover {
                background: ${fill};
            }
        }
    });

    html! {
        <i
            ref={icon_ref}
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
