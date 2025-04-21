use crate::hooks::use_theme;
use stylist::yew::styled_component;
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

    let theme = use_theme();
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
        background-color: ${theme.color.display_rgb()};
        color: ${theme.background_color.display_rgb()};
        cursor: pointer;
        border: 0;

        &:hover {
            background-color: ${ theme.color.scale(1.5).display_rgb() };
        }
    };

    let email_input_css = css! {
        font-family: inherit;
        font-size: 18px;
        background-color: ${theme.background_color.display_rgb()};
        color: ${theme.color.display_rgb()};
        border-radius: ${BORDER_RADIUS};
        width: ${WIDTH};
        height: ${HEIGHT};
        box-sizing: border-box;
        display: flex;
        border: ${BORDER_WIDTH};
        border-style: solid;
        border-color: ${theme.color.display_rgb()};
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
            </div>
        }
    }
}
