use crate::hooks::use_theme;
use stylist::yew::styled_component;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ProfilePictureProps {
    pub src: AttrValue,
}

#[styled_component]
pub fn ProfilePicture(props: &ProfilePictureProps) -> Html {
    let theme = use_theme();

    const BORDER_WIDTH: &str = "3px";

    let img_css = css! {
        border-radius: 50%;
        width: 300px;
        max-width: 600px;
        height: 300px;
        max-height: 600px;
        object-fit: scale-down;

        /* Outline */
        border-width: ${BORDER_WIDTH};
        border-style: solid;
        border-color: ${theme.color.display_rgb()};
    };

    html! {
        <img
            alt="Spencer C. Imbleau"
            width="300px"
            height="300px"
            class={img_css}
            src={ props.src.clone() }
        />
    }
}
