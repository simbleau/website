use stylist::css;
use yew::prelude::*;

use crate::style::colors::Color;
use crate::style::icons::{Icon, IconMask};
use crate::style::themes::{use_theme, ThemeChoice};

const MIN_BG_SIZE: &str = "32px";
const MIN_FG_SIZE: &str = "16px";

#[derive(Properties, PartialEq)]
pub struct Props {
    pub mask: IconMask,
    #[prop_or_default]
    pub color: Option<Color>,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

#[function_component(TapTarget)]
pub fn view(props: &Props) -> Html {
    let theme = use_theme();

    // Coloring
    let bgc = theme.fg1.with_alpha(0.10);
    let bgc_hover = theme.fg1.with_alpha(0.25);
    let fgc = match props.color {
        Some(c) => c,
        None => theme.fg1,
    };
    let fgc_hover = match theme.kind() {
        ThemeChoice::Dark => fgc.lighten(0.3),
        ThemeChoice::Light => fgc.darken(0.3),
    };

    // Sizing - Background
    let bgh = format!("max({}, calc({} * 1.5))", MIN_BG_SIZE, theme.fs);
    let bgw = bgh.clone();

    // Sizing - Foreground
    let fgh = format!("max({}, {})", MIN_FG_SIZE, theme.fs);
    let fgw = format!("calc({} * {})", fgh, IconMask::aspect_ratio(props.mask));
    let fgh_hover = format!("calc({} * 1.3)", fgh);
    let fgw_hover = format!("calc({} * 1.3)", fgw);

    // Sizing - Mobile
    let mobile_aspect = "1.04"; // TODO: Don't do this... Should be (theme.fsm / theme.fs)
    let bghm = format!("calc({} * {})", bgh, mobile_aspect);
    let bgwm = format!("calc({} * {})", bgw, mobile_aspect);
    let fghm = format!("calc({} * {})", fgh, mobile_aspect);
    let fgwm = format!("calc({} * {})", fgw, mobile_aspect);
    let fghm_hover = format!("calc({} * {})", fgh_hover, mobile_aspect);
    let fgwm_hover = format!("calc({} * {})", fgw_hover, mobile_aspect);

    // Sizing - Tablet
    let tablet_aspect = "1.08"; // TODO: Don't do this... Should be (theme.fst / theme.fs)
    let bght = format!("calc({} * {})", bgh, tablet_aspect);
    let bgwt = format!("calc({} * {})", bgw, tablet_aspect);
    let fght = format!("calc({} * {})", fgh, tablet_aspect);
    let fgwt = format!("calc({} * {})", fgw, tablet_aspect);
    let fght_hover = format!("calc({} * {})", fgh_hover, tablet_aspect);
    let fgwt_hover = format!("calc({} * {})", fgw_hover, tablet_aspect);

    // Sizing - Desktop
    let desktop_aspect = "1.12"; // TODO: Don't do this... Should be (theme.fsd / theme.fs)
    let bghd = format!("calc({} * {})", bgh, desktop_aspect);
    let bgwd = format!("calc({} * {})", bgw, desktop_aspect);
    let fghd = format!("calc({} * {})", fgh, desktop_aspect);
    let fgwd = format!("calc({} * {})", fgw, desktop_aspect);
    let fghd_hover = format!("calc({} * {})", fgh_hover, desktop_aspect);
    let fgwd_hover = format!("calc({} * {})", fgw_hover, desktop_aspect);

    let style = css!(
        r#"
        & {
            display:block;
            cursor:pointer;

            width: ${bgw};
            height: ${bgh};
            border: 0;
            border-radius: 50%;
            text-align:center;

            transition: background-color 0.5s;
            background-color: ${bgc};
        }

        &:hover {
            background-color: ${bgc_hover};
        }

        & > i {
            transition: width 0.5s, height 0.5s;
            background: ${fgc};
            width: ${fgw};
            height: ${fgh};
        }

        &:hover > i {
            background: ${fgc_hover};
            width: ${fgw_hover};
            height: ${fgh_hover};
        }

        @media (min-width: 768px) {
            & {
                width: ${bgwm};
                height: ${bghm};
            }
            & > i {
                width: ${fgwm};
                height: ${fghm};
            }
            &:hover > i {
                width: ${fgwm_hover};
                height: ${fghm_hover};
            }
        }

        @media (min-width: 992px) {
            & {
                width: ${bgwt};
                height: ${bght};
            }
            & > i {
                width: ${fgwt};
                height: ${fght};
            }
            &:hover > i {
                width: ${fgwt_hover};
                height: ${fght_hover};
            }
        }

        @media (min-width: 1200px) {
            & {
                width: ${bgwd};
                height: ${bghd};
            }
            & > i {
                width: ${fgwd};
                height: ${fghd};
            }
            &:hover > i {
                width: ${fgwd_hover};
                height: ${fghd_hover};
            }
        }

        "#,
        /* Coloring */
        bgc = bgc,
        bgc_hover = bgc_hover,
        fgc = fgc,
        fgc_hover = fgc_hover,
        /* Sizing - Default */
        bgw = bgw,
        bgh = bgh,
        fgw = fgw,
        fgh = fgh,
        fgw_hover = fgw_hover,
        fgh_hover = fgh_hover,
        /* Sizing - Mobile */
        bgwm = bgwm,
        bghm = bghm,
        fgwm = fgwm,
        fghm = fghm,
        fgwm_hover = fgwm_hover,
        fghm_hover = fghm_hover,
        /* Sizing - Tablet */
        bgwt = bgwt,
        bght = bght,
        fgwt = fgwt,
        fght = fght,
        fgwt_hover = fgwt_hover,
        fght_hover = fght_hover,
        /* Sizing - Desktop */
        bgwd = bgwd,
        bghd = bghd,
        fgwd = fgwd,
        fghd = fghd,
        fgwd_hover = fgwd_hover,
        fghd_hover = fghd_hover,
    );

    let icon_style = css!(
        r#"
        & {
            pointer-events: none;
        }
        "#,
    );

    html! {
        <button class={style} onclick={ props.onclick.clone() } >
        <Icon
            mask={ props.mask }
            class={ icon_style }
        />
        </button>
    }
}
