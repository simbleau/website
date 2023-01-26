use crate::components::ProfilePicture;
use accessible_ui::prelude::*;
use stylist::yew::styled_component;
use yew::prelude::*;

#[styled_component]
pub fn HomePage() -> Html {
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

    html! {
        <div align="center" class={container_style}>
            <ProfilePicture src={"/static/images/me.webp"} />
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
