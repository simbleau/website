use crate::style::themes::ThemeChoice;
use stylist::yew::styled_component;
use themer::prelude::*;
use yew::{prelude::*, virtual_dom::AttrValue};

#[derive(Properties, PartialEq)]
pub struct ProfilePictureProps {
    pub src: AttrValue,
}

#[styled_component]
pub fn ProfilePicture(props: &ProfilePictureProps) -> Html {
    let theme = use_theme::<ThemeChoice>();

    let img_css = css! {
        r#"
            border-radius: 50%;
            width: 300px;
            max-width: 600px;
            height: 300px;
            max-height: 600px;
            object-fit: scale-down;

            /* Outline */
            border: solid 3px ${border};
        "#,
        border = theme.color,
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
