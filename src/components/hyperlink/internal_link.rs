use crate::components::{Icon, IconMask};
use crate::router::Route;
use stylist::yew::styled_component;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct RouterLinkProps {
    pub to: Route,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub icon: Option<IconMask>,
}

#[styled_component(InternalLink)]
pub fn view(props: &RouterLinkProps) -> Html {
    html! {
        <Link<Route> to={ props.to }>
            if let Some(mask) = props.icon {
                <Icon
                    data_id="linkicon"
                    mask={mask}
                    class={classes!(css!("vertical-align: middle; margin-right: 3px;"))}
                />
            }
            { props.children.clone() }
        </Link<Route>>
    }
}
