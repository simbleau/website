use crate::util::lighten;
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
        position: relative;
        border-radius: ${BORDER_RADIUS};
        width: ${WIDTH};
        height: ${HEIGHT};
        background-color: ${theme.color};
        color: ${theme.background_color};
        cursor: pointer;
        border-width: 3px;
        border-style: solid;
        border-color: ${theme.color};
    };

    let tooltip_css = css! {
        & {
            position: relative;
        }

        & > #wrapper {
            position: absolute;
            left: 75px;
            top: -10px;
            border: 1px solid green;
            text-align: center;
            align-items: center;
            display: inline-flex;
        }
        & > #wrapper > *[data-aui-id="help"] {
            margin: 10px;
        }

        &::before {
            content: attr(data-tooltip);
            width: 200px;
            display: none;
            position: absolute;
            background-color: #333;
            color: #fff;
            padding: 5px;
            border-radius: 5px;
            left: 250px;
            transform: translateX(-50%);
        }

        &:hover::before {
            display: block;
        }
    };

    let email_ctr_css = css! {
        width: ${WIDTH};
        height: ${HEIGHT};
        box-sizing: border-box;
    };

    let email_input_css = css! {
        background-color: ${theme.background_color};
        color: ${theme.color};
        border-top-left-radius: ${BORDER_RADIUS};
        border-bottom-left-radius: ${BORDER_RADIUS};
        width: ${format!("calc(100% - {})", COPY_BUTTON_WIDTH)};
        height: 100%;
        border-top-width: 3px;
        border-bottom-width: 3px;
        border-left-width: 3px;
        border-right-width: 0;
        border-style: solid;
        border-color: ${theme.color};
        box-sizing: border-box;
        vertical-align: baseline;
        text-align: center;
        margin: 0;
    };

    let copy_button_css = css! {
        background-color: ${theme.background_color};
        color: ${theme.color};
        border-top-right-radius: ${BORDER_RADIUS};
        border-bottom-right-radius: ${BORDER_RADIUS};
        width: ${COPY_BUTTON_WIDTH};
        height: 100%;
        border-width: 3px;
        border-style: solid;
        border-color: ${theme.color};
        box-sizing: border-box;
        margin: 0;

        &:hover {
            background-color: ${lighten(&theme.background_color, 1.2)};
            color: ${lighten(&theme.color, 1.2)};
        }

        &:hover *[data-aui-id="copy"] {
            background: ${lighten(&theme.color, 1.2)};
        }
    };

    let onreveal = {
        let put_email = props.email.clone();
        let email = email.clone();
        let visible = visible.clone();
        Callback::from(move |_| {
            email.set(put_email.to_string());
            visible.set(true);
        })
    };

    let oncopy = {
        let put_email = props.email.clone().to_string();
        Callback::from(move |_| {
            let clipboard = web_sys::window()
                .expect("window")
                .navigator()
                .clipboard()
                .unwrap();
            // Todo: pop an info/error toast depending on result
            _ = clipboard.write_text(&put_email);
        })
    };

    html! {
        if *visible {
            <div class={email_ctr_css}>
                <input
                    class={email_input_css}
                    value={email.to_string()}
                    readonly=true
                />
                <button
                    onclick={oncopy}
                    class={copy_button_css}
                >
                    <Icon mask={IconMask::CopyToClipboard} data_aui_id={"copy"} />
                </button>
            </div>
        } else {
            <div>
                <button onclick={onreveal} class={btn_css}>
                    <span>
                        {"reveal email"}
                    </span>
                    <span
                        class={tooltip_css}
                        data-tooltip="This helps me fight bots!">
                        <span id="wrapper">
                            <Icon
                                mask={IconMask::Question}
                                fill={theme.background_color}
                                data_aui_id={"help"}
                            />
                        </span>
                    </span>
                </button>

            </div>
        }
    }
}
