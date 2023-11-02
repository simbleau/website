use crate::{
    components::{Icon, IconMask},
    style::themes::ThemeChoice,
    util::lighten,
};
use futures_util::FutureExt;
use stylist::yew::{styled_component, use_media_query};
use themer::yew::use_theme;
use wasm_bindgen_futures::JsFuture;
use yew::prelude::*;

#[derive(Properties, PartialEq, Eq)]
pub struct EmailButtonProps {
    pub user: AttrValue,
    pub domain: AttrValue,
}

#[styled_component]
pub fn EmailButton(props: &EmailButtonProps) -> Html {
    const WIDTH: &str = "300px";
    const COPY_BUTTON_WIDTH: &str = "50px";
    const HEIGHT: &str = "50px";
    const BORDER_RADIUS: &str = "10px";
    const BORDER_WIDTH: &str = "3px";

    let is_mobile = use_media_query("(max-width: 768px)");
    let theme = use_theme::<ThemeChoice>();
    let email = use_state(|| "".to_string());
    let visible = use_state(|| false);

    // I assemble the email like this because there are bots on GitHub that
    // scrape emails, too. Less common that web crawlers, but they still exist.
    // In other words, I don't want my 'example@domain.com' anywhere.
    let real_email = format!("{}@{}", props.user, props.domain);

    let btn_css = css! {
        position: relative;
        border-radius: ${BORDER_RADIUS};
        width: ${WIDTH};
        height: ${HEIGHT};
        background-color: ${theme.color};
        color: ${theme.background_color};
        cursor: pointer;
        border: 0;

        &:hover {
            background-color: ${lighten(&theme.color, 1.2)};
        }
    };

    let tooltip_css = css! {
        & {
            position: relative;
        }

        & > #wrapper {
            cursor: help;
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
            background-color: ${theme.color};
            color: ${theme.background_color};
            padding: 10px;
            border-radius: 5px;
            left: 230px;
            top: -40px;
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
        border-top-width: ${BORDER_WIDTH};
        border-bottom-width: ${BORDER_WIDTH};
        border-left-width: ${BORDER_WIDTH};
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
        border-width: ${BORDER_WIDTH};
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
        let put_email = real_email.clone();
        let email = email.clone();
        let visible = visible.clone();
        Callback::from(move |_| {
            email.set(put_email.to_string());
            visible.set(true);
        })
    };

    let oncopy = {
        let put_email = real_email.clone();
        Callback::from(move |_| {
            let Some(clipboard) =
                web_sys::window().expect("window").navigator().clipboard()
            else {
                // TODO: Error toast
                return;
            };
            let copy_task = JsFuture::from(clipboard.write_text(&put_email))
                .then(|result| async move {
                    match result {
                        Ok(_) => {
                            // Successfully copied to clipboard
                            // TODO: Success toast
                        }
                        Err(err) => {
                            // TODO: Error toast
                        }
                    }
                });
            wasm_bindgen_futures::spawn_local(copy_task);
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
                    if !is_mobile {
                        <span
                            class={tooltip_css}
                            data-tooltip="an anti-spam measure">
                            <span id="wrapper">
                                <Icon
                                    mask={IconMask::Question}
                                    fill={theme.background_color}
                                    data_aui_id={"help"}
                                />
                            </span>
                        </span>
                    }
                </button>

            </div>
        }
    }
}
