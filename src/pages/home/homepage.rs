use stylist::css;
use yew::prelude::*;

use crate::components::TapTarget;
use crate::style::icons::IconMask;
use crate::style::themes::use_theme;

#[function_component(HomePage)]
pub fn view() -> Html {
    let theme = use_theme();

    let container_style = css!(
        r#"
            margin: 10px 0 0 0;
            animation: size-anim 0.5s ease;
            @keyframes size-anim {
                from {
                    transform: scaleX(0);
                    opacity: 0;
                }
                to {
                    transform: scaleX(1);
                    opacity: 1;
                }
            }
        "#
    );

    let shadow = match theme.kind() {
        crate::style::themes::ThemeChoice::Dark => "rgba(0,0,0,0.5)",
        crate::style::themes::ThemeChoice::Light => "rgba(0,0,0,0.25)",
    };
    let image_style = css!(
        r#"
        width: 300px;
        height: auto;
        max-width: 80%;
        border-radius: 50%;
        box-shadow: 0 0 10px ${shadow};
        "#,
        shadow = shadow
    );

    html! {
        <div align="center" class={container_style}>
            <img    src="/static/images/Me.jpeg"
                    alt="Spencer C. Imbleau"
                    class={ image_style }
            />
            <br />
            <div class={ css!("display: inline-flex; & > * {margin: 0 5px;}") }>
                <a href="https://www.linkedin.com/in/simbleau/">
                    <TapTarget mask={IconMask::LinkedIn} />
                </a>
                <a href="https://www.twitter.com/spencerimbleau/">
                    <TapTarget mask={IconMask::Twitter} />
                </a>
                <a href="https://www.github.com/simbleau/">
                    <TapTarget mask={IconMask::GitHub} />
                </a>
            </div>
        </div>
    }
}
