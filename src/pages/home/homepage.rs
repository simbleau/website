use crate::components::IconMask;
use crate::components::TapTarget;
use crate::style::themes::BrandChoice;
use cssugar::prelude::*;
use stylist::css;
use themer::prelude::*;
use yew::prelude::*;

#[function_component(HomePage)]
pub fn view() -> Html {
    let theme = use_theme::<BrandChoice>();

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
        BrandChoice::Dark => BLACK.alpha(0.5),
        BrandChoice::Light => BLACK.alpha(0.25),
    };
    let image_style = css!(
        r#"
        width: 300px;
        height: auto;
        max-width: 80%;
        max-height: 300px;
        border-radius: 50%;
        box-shadow: 0 0 10px ${shadow};
        "#,
        shadow = shadow
    );

    html! {
        <div align="center" class={container_style}>
            <img    width="300" height="300"
                    src="/static/images/me.webp"
                    alt="Spencer C. Imbleau"
                    class={ image_style }
            />
            <br />
            <div class={ css!("display: inline-flex; & > * {margin: 10px 5px;}") }>
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
