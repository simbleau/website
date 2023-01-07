use crate::components::{Icon, IconMask};
use stylist::yew::styled_component;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ExternalLinkProps {
    pub url: AttrValue,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub icon: Option<IconMask>,
}

#[styled_component(ExternalLink)]
pub fn view(props: &ExternalLinkProps) -> Html {
    html! {
        <a href={ props.url.clone() }>
            if let Some(mask) = props.icon {
                <Icon
                    data_id="linkicon"
                    mask={mask}
                    class={classes!(css!("vertical-align: middle; margin-right: 3px;"))}
                />
            }
            { props.children.clone() }
            <Icon
                data_id="linkicon"
                mask={ IconMask::Share }
                class={classes!(css!("vertical-align: baseline !important; margin-left: 3px;"))}
                scale={ 0.75 }
            />
        </a>
    }
}
