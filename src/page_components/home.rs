use crate::components::{EmailButton, Footer, IconMask, ProfilePicture, TapTarget};
use stylist::yew::styled_component;
use yew::prelude::*;

#[styled_component(HomePage)]
pub fn view() -> Html {
    let container_style = css!(
        r#"
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
            <h1>{"Spencer C. Imbleau"}</h1>
            <EmailButton email_base64={"c3BlbmNlckBpbWJsZWF1LmNvbQ=="} />
            <br/>
            <div class={ css!("display: inline-flex; & > * {margin: 0 10px;}") }>
                <a href="https://www.linkedin.com/in/simbleau/" target="_blank" alt="Go to LinkedIn">
                    <TapTarget mask={IconMask::LinkedIn} aria_label="LinkedIn" />
                </a>
                <a rel="me" href="https://mastodon.online/@scim" target="_blank" alt="Go to Mastodon">
                    <TapTarget mask={IconMask::Mastodon} aria_label="Mastodon" />
                </a>
                <a href="https://www.github.com/simbleau/" target="_blank" alt="Go to GitHub">
                    <TapTarget mask={IconMask::GitHub} aria_label="GitHub" />
                </a>
            </div>
            <Footer />
        </div>
    }
}
