use stylist::yew::styled_component;
use themer::prelude::*;
use yew::prelude::*;

use crate::style::themes::BrandChoice;

#[derive(Properties, PartialEq)]
pub struct PreformattedProps {
    pub inner: String,
}

#[styled_component(Preformatted)]
pub fn view(props: &PreformattedProps) -> Html {
    let theme = use_theme::<BrandChoice>();

    let pre_css = css! {
        r#"
            white-space: pre-wrap;
            border-radius: 3px;
            background-color: ${link_alpha};
            color: ${link};
        "#,
        link = theme.link,
        link_alpha = theme.link.alpha(0.10),
    };

    html! {
        <span class={classes!("preformatted", pre_css)}>
            { props.inner.clone() }
        </span>
    }
}
