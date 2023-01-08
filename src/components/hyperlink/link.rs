use crate::components::hyperlink::{ExternalLink, InternalLink};
use crate::components::IconMask;
use crate::router::Route;
use crate::style::themes::ThemeChoice;
use stylist::yew::styled_component;
use themer::prelude::*;
use yew::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Destination {
    Internal(Route),
    External(&'static str),
}

#[derive(Properties, PartialEq)]
pub struct HyperlinkProps {
    pub to: Destination,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub icon: Option<IconMask>,
}

#[styled_component(Hyperlink)]
pub fn view(props: &HyperlinkProps) -> Html {
    let theme = use_theme::<ThemeChoice>();
    let style = css! {
        & {
            display: inline-block;
        }
        & *[data-id="linkicon"] {
            background-color: ${theme.link};
        }
        &:hover *[data-id="linkicon"] {
            background-color: ${theme.link_hover};
        }
    };

    html! {
        <div class={style}>
        {
            match &props.to {
                Destination::Internal(route) => html! {
                    <InternalLink
                        to={*route}
                        icon={props.icon}
                    >
                        { props.children.clone() }
                    </InternalLink>
                },
                Destination::External(url) => html! {
                    <ExternalLink
                        url={*url}
                        icon={props.icon}
                    >
                        { props.children.clone() }
                    </ExternalLink>
                },
            }
        }
        </div>
    }
}
