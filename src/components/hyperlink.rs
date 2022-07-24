use std::fmt::Display;

use stylist::yew::styled_component;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::Route;
use crate::style::icons::{Icon, IconMask};
use crate::style::themes::use_theme;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Url {
    Local(Route),
    External(&'static str),
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub domain: Url,
    pub display: Html,
    #[prop_or_default]
    pub icon: Option<IconMask>,
}

#[styled_component(Hyperlink)]
pub fn view(props: &Props) -> Html {
    let theme = use_theme();

    match &props.domain {
        Url::Local(route) => html! {
            <Link<Route> to={ *route }>
                if let Some(mask) = props.icon {
                    <Icon {mask} fill={theme.ac1} />
                }
                { props.display.clone() }
            </Link<Route>>
        },
        Url::External(url) => html! {
            <a href={ *url }>
                if let Some(mask) = props.icon {
                    <Icon {mask} fill={theme.ac1} />
                }
                { props.display.clone() }
                <Icon
                    mask={ IconMask::Share }
                    class={ css!("vertical-align: top !important;") }
                />
            </a>
        },
    }
}
