use stylist::yew::styled_component;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::style::icons::Icon;
use crate::style::icons::IconMask;
use crate::{router::Route, style::themes::use_theme};

use super::hyperlink::Url;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub domain: Url,
    pub display: Html,
}

#[styled_component(NavLink)]
pub fn view(props: &Props) -> Html {
    let theme = use_theme();

    let style = css! {
        r#"
        & {
            display: flex;
            flex-direction: column;
            justify-content: center;
            align-items: center;
            padding-bottom: 5px;
        }
        & > #underline {
            height: 3px;
            width: 0%;

            transition: width 0.2s ease-out, background-color 0.5s;
            background-color: ${ac1};
        }
        &:hover > #underline {
            width: 100%;
            background-color: ${ac2};
        }
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
                    { props.display.clone() }
                    <div id="underline" />
                </div>
            </Link<Route>>
        },
        Url::External(url) => html! {
            <a href={ *url }>
                <div class={style}>
                    <div>
                        { props.display.clone() }
                        <Icon mask={ IconMask::Share }/>
                    </div>
                    <div id="underline" />
                </div>
            </a>
        },
    }
}
