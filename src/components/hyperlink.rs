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

    let style = css! {
        r#"
        & i {
            vertical-align: top !important;
            margin-left: 3px;

            background-color: ${ac1};
        }
        &:hover i {
            background-color: ${ac2};
        }
        "#,
        ac1 = theme.ac1,
        ac2 = theme.ac2,
    };

    match &props.domain {
        Url::Local(route) => html! {
            <Link<Route> to={ *route }>
                <div class={style}>
                    if let Some(mask) = props.icon {
                        <Icon {mask} />
                    }
                    { props.display.clone() }
                </div>
            </Link<Route>>
        },
        Url::External(url) => html! {
            <a href={ *url }>
                <div class={style}>
                    if let Some(mask) = props.icon {
                        <Icon {mask} />
                        {" "}
                    }
                    { props.display.clone() }
                    <Icon
                        mask={ IconMask::Share }
                    />
                </div>
            </a>
        },
    }
}
