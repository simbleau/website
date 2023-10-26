use crate::components::{ExternalLink, InternalLink};
use crate::router::Route;
use crate::style::themes::ThemeChoice;
use stylist::yew::styled_component;
use themer::prelude::*;
use yew::prelude::*;

#[derive(PartialEq)]
pub enum NavDestination {
    Internal(Route),
    External(AttrValue),
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub to: NavDestination,
    pub children: Children,
}

#[styled_component]
pub fn NavLink(props: &Props) -> Html {
    let theme = use_theme::<ThemeChoice>();

    let link_css = css! {
        & > a > span > #underline {
            height: 3px;
            width: 0%;

            transition: width 0.2s ease-out, background-color 0.5s;
            background-color: ${theme.link};
        }
        &:hover > a > span > #underline {
            width: 100%;
            background-color: ${theme.link_hover};
        }
    };

    let underline_ctr_style = css! {
        display: flex;
        flex-direction: column;
        align-items: center;
    };

    match &props.to {
        NavDestination::Internal(route) => html! {
            <InternalLink<Route> to={*route} class={link_css}>
                <span class={underline_ctr_style}>
                    { props.children.clone() }
                    <div id="underline" />
                </span>
            </InternalLink<Route>>
        },
        NavDestination::External(url) => html! {
            <ExternalLink to={url}class={link_css}>
                <span class={underline_ctr_style}>
                    { props.children.clone() }
                    <div id="underline" />
                </span>
            </ExternalLink>
        },
    }
}
