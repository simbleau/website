use crate::components::{Destination, Icon, IconMask};
use crate::router::Route;
use crate::style::themes::ThemeChoice;
use stylist::yew::styled_component;
use themer::prelude::*;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub to: Destination,
    pub children: Children,
}

#[styled_component(NavLink)]
pub fn view(props: &Props) -> Html {
    let theme = use_theme::<ThemeChoice>();

    let style = css! {
        r#"
        & {
            display: flex;
            flex-direction: column;
            align-items: center;
            padding-bottom: 5px;
        }
        & > #underline {
            height: 3px;
            width: 0%;

            transition: width 0.2s ease-out, background-color 0.5s;
            background-color: ${link};
        }
        &:hover > #underline {
            width: 100%;
            background-color: ${link_hover};
        }
        & > div > i {
            vertical-align: baseline;
            margin-left: 3px;

            background-color: ${link};
        }
        &:hover > div > i {
            background-color: ${link_hover};
        }
        "#,
        link = theme.link,
        link_hover = theme.link_hover,
    };

    match &props.to {
        Destination::Internal(route) => html! {
            <Link<Route> to={ *route }>
                <div class={style}>
                    { props.children.clone() }
                    <div id="underline" />
                </div>
            </Link<Route>>
        },
        Destination::External(url) => html! {
            <a href={ *url }>
                <div class={style}>
                    <div>
                        { props.children.clone() }
                        <Icon
                            mask={ IconMask::Share }
                            scale={ 0.75 }
                        />
                    </div>
                    <div id="underline" />
                </div>
            </a>
        },
    }
}
