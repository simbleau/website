use crate::components::IconMask;
use crate::style::themes::ThemeChoice;
use cssugar::prelude::*;
use stylist::css;
use themer::prelude::*;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub data_id: Option<AttrValue>,
    pub mask: IconMask,
    #[prop_or_default]
    pub fill: Option<Color>,
    #[prop_or_default]
    pub hover_fill: Option<Color>,
    #[prop_or_default]
    pub scale: Option<f32>,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(Icon)]
pub fn icon(props: &Props) -> Html {
    let theme = use_theme::<ThemeChoice>();

    // Sizing
    let fsh = format!("calc({} * {})", props.scale.unwrap_or(1.0), theme.fs);
    let fsw = format!("calc({} * {})", fsh, IconMask::aspect_ratio(props.mask));

    // Size - Mobile
    let fshm = format!("calc({} * {})", props.scale.unwrap_or(1.0), theme.fsm);
    let fswm =
        format!("calc({} * {})", fshm, IconMask::aspect_ratio(props.mask));

    // Size - Tablet
    let fsht = format!("calc({} * {})", props.scale.unwrap_or(1.0), theme.fst);
    let fswt =
        format!("calc({} * {})", fsht, IconMask::aspect_ratio(props.mask));

    // Size - Desktop
    let fshd = format!("calc({} * {})", props.scale.unwrap_or(1.0), theme.fsd);
    let fswd =
        format!("calc({} * {})", fshd, IconMask::aspect_ratio(props.mask));

    let icon_style = css! {
        r#"
        & {
            width: ${fsw};
            height: ${fsh};
            display: inline-block;
            text-align: center;
            vertical-align: middle;
            text-decoration: none;
        }
        @media (min-width: 768px) {
            & {
                width: ${fswm};
                height: ${fshm};
            }
        }
        @media (min-width: 992px) {
            & {
                width: ${fswt};
                height: ${fsht};
            }
        }
        @media (min-width: 1200px) {
            & {
                width: ${fswd};
                height: ${fshd};
            }
        }
        "#,
        fsw = fsw,
        fsh = fsh,
        fswm = fswm,
        fshm = fshm,
        fswt = fswt,
        fsht = fsht,
        fswd = fswd,
        fshd = fshd,
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
        fill = props.fill.unwrap_or(theme.color),
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
        <i
            data-id={props.data_id.clone()}
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
