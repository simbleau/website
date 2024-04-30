use crate::{
    components::{EmailButton, IconMask, ProfilePicture, TapTarget},
    footer::Footer,
};
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
            <EmailButton user={"spencer"} domain={"imbleau.com"} />
            <br/>
            <div class={ css!("display: inline-flex; & > * {margin: 0 10px;}") }>
                <a href="https://www.linkedin.com/in/simbleau/" target="_blank">
                    <TapTarget mask={IconMask::LinkedIn} />
                </a>
                <a rel="me" href="https://mastodon.online/@scim" target="_blank">
                    <TapTarget mask={IconMask::Mastodon} />
                </a>
                <a href="https://www.github.com/simbleau/" target="_blank">
                    <TapTarget mask={IconMask::GitHub} />
                </a>
            </div>
            <Footer />
        </div>
    }
}
