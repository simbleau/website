use crate::{
    components::{Icon, IconMask},
    style::themes::ThemeChoice,
};
use stylist::yew::styled_component;
use themer::yew::use_theme;
use yew::prelude::*;

#[derive(Properties, PartialEq, Eq)]
pub struct EmailButtonProps {
    pub email: AttrValue,
}

#[styled_component]
pub fn EmailButton(props: &EmailButtonProps) -> Html {
    let theme = use_theme::<ThemeChoice>();
    let email = use_state(|| "".to_string());
    let visible = use_state(|| false);

    const WIDTH: &str = "300px";
    const COPY_BUTTON_WIDTH: &str = "50px";
    const HEIGHT: &str = "40px";
    const BORDER_RADIUS: &str = "5px";

    let btn_css = css! {
        border-radius: ${BORDER_RADIUS};
        width: ${WIDTH};
        height: ${HEIGHT};
        background-color: ${theme.color};
        color: ${theme.background_color};
        cursor: pointer;
        border-width: 3px;
        border-style: outset solid;
        border-color: ${theme.color};
    };

    let email_ctr_css = css! {
        width: ${WIDTH};
        height: ${HEIGHT};
        box-sizing: border-box;
    };

    let email_css = css! {
        border-top-left-radius: ${BORDER_RADIUS};
        border-bottom-left-radius: ${BORDER_RADIUS};
        width: ${format!("calc(100% - {})", COPY_BUTTON_WIDTH)};
        height: 100%;
        border-top-width: 3px;
        border-bottom-width: 3px;
        border-left-width: 3px;
        border-right-width: 0;
        border-style: outset solid;
        border-color: ${theme.color};
        box-sizing: border-box;
        vertical-align: baseline;
        margin: 0;
    };

    let copy_css = css! {
        border-top-right-radius: ${BORDER_RADIUS};
        border-bottom-right-radius: ${BORDER_RADIUS};
        width: ${COPY_BUTTON_WIDTH};
        height: 100%;
        background_color: ${theme.color.with_a(20)};
        border-width: 3px;
        border-style: outset solid;
        border-color: ${theme.color};
        box-sizing: border-box;
        margin: 0;
    };

    let onclick = {
        let put_email = props.email.clone();
        let email = email.clone();
        let visible = visible.clone();
        Callback::from(move |_| {
            email.set(put_email.to_string());
            visible.set(true);
        })
    };

    html! {
        if *visible {
            <div class={email_ctr_css}>
                <input
                    class={email_css}
                    value={email.to_string()}
                    readonly=true
                />
                <button
                    class={copy_css}
                >
                    <Icon mask={IconMask::CopyToClipboard} />
                </button>
            </div>
        } else {
            <div>
                <button {onclick} class={btn_css}>
                    <span>
                        {"Show email"}
                    </span>
                </button>
            </div>
        }
    }
}
