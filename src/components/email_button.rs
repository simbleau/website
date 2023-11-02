use crate::{
    components::{Icon, IconMask},
    style::themes::ThemeChoice,
    util::lighten,
};
use stylist::yew::{styled_component, use_media_query};
use themer::yew::use_theme;
use yew::prelude::*;

#[derive(Properties, PartialEq, Eq)]
pub struct EmailButtonProps {
    pub user: AttrValue,
    pub domain: AttrValue,
}

#[styled_component]
pub fn EmailButton(props: &EmailButtonProps) -> Html {
    const WIDTH: &str = "300px";
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
        font-family: inherit;
        font-size: inherit;
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
            font-size: 24px;
        }

        & > #wrapper {
            cursor: help;
            position: absolute;
            left: -30px;
            top: 50%;
            text-align: center;
            align-items: center;
            display: inline-flex;
            transform: translateX(-50%) translateY(-50%);
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
            left: 120px;
            top: -0px;
            transform: translateX(-50%) translateY(-100%);
        }

        &:hover::before {
            display: block;
        }
    };

    let email_input_css = css! {
        font-family: inherit;
        font-size: 18px;
        background-color: ${theme.background_color};
        color: ${theme.color};
        border-radius: ${BORDER_RADIUS};
        width: ${WIDTH};
        height: ${HEIGHT};
        box-sizing: border-box;
        display: flex;
        border: ${BORDER_WIDTH};
        border-style: solid;
        border-color: ${theme.color};
        vertical-align: baseline;
        text-align: center;
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

    html! {
        if *visible {
            <input
                class={email_input_css}
                value={email.to_string()}
                readonly=true
            />
        } else {
            <div>
                <button onclick={onreveal} class={btn_css}>
                    <span>
                        {"reveal email"}
                    </span>
                </button>
                if !is_mobile {
                    <span
                        class={tooltip_css}
                        data-tooltip="my email is shy">
                        <span id="wrapper">
                            <Icon
                                mask={IconMask::Question}
                                fill={theme.background_color}
                                data_aui_id={"help"}
                            />
                        </span>
                    </span>
                }
            </div>
        }
    }
}
