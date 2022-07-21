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
                    opacity: 0;
                }
                to {
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
        height: auto;
        width: auto;
        max-width: 80%;
        max-height: 300px;
        border-radius: 50%;
        box-shadow: 0 0 10px ${shadow};
        "#,
        shadow = shadow
    );

    html! {
        <div align="center" class={container_style}>
            <img    width="800" height="800"
                    src="/static/images/me.webp"
                    alt="Spencer C. Imbleau"
                    class={ image_style }
            />
            <br />
            <div class={ css!("display: inline-flex; & > * {margin: 10px 5px;}") }>
                <a href="https://www.linkedin.com/in/simbleau/">
                    <TapTarget accessibility={"LinkedIn"} mask={IconMask::LinkedIn} />
                </a>
                <a href="https://www.twitter.com/spencerimbleau/">
                    <TapTarget accessibility={"Twitter"} mask={IconMask::Twitter} />
                </a>
                <a href="https://www.github.com/simbleau/">
                    <TapTarget accessibility={"GitHub"} mask={IconMask::GitHub} />
                </a>
            </div>
        </div>
    }
}
