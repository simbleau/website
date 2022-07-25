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
struct ExternalLinkProps {
    url: &'static str,
    display: Html,
    #[prop_or_default]
    icon: Option<IconMask>,
}

#[styled_component(ExternalLink)]
fn view_external_link(props: &ExternalLinkProps) -> Html {
    html! {
        <a href={ props.url }>
            if let Some(mask) = props.icon {
                <Icon {mask} />
            }
            { props.display.clone() }
            <Icon
                mask={ IconMask::Share }
                scale={ 0.75 }
            />
        </a>
    }
}

#[derive(Properties, PartialEq)]
struct RouteLinkProps {
    route: Route,
    display: Html,
    #[prop_or_default]
    icon: Option<IconMask>,
}

#[styled_component(RouteLink)]
fn view_route_link(props: &RouteLinkProps) -> Html {
    html! {
        <Link<Route> to={ props.route }>
            if let Some(mask) = props.icon {
                <Icon {mask} />
            }
            { props.display.clone() }
        </Link<Route>>
    }
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

    let style = css! {
        r#"
        & {
            display: inline;
        }
        a {
            display: inline-flex;
            align-items: center;
        }
        a > i {
            vertical-align: baseline;
            margin-left: 3px;

            background-color: ${ac1};
        }
        a:hover > i {
            background-color: ${ac2};
        }
        "#,
        ac1 = theme.ac1,
        ac2 = theme.ac2,
    };
    html! {
        <div class={style}>
        {
            match &props.domain {
                Url::Local(route) => html! {
                <RouteLink
                    route={*route}
                    display={props.display.clone()}
                    icon={props.icon}
                />
            },
                Url::External(url) => html! {
                    <ExternalLink
                        url={*url}
                        display={props.display.clone()}
                        icon={props.icon}
                    />
                },
            }
        }
        </div>
    }
}
