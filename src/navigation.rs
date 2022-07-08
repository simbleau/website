use std::fmt;

use stylist::yew::styled_component;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::style::icons::{Icon, IconMask};
use crate::{router::Route, style::use_theme};

#[derive(Clone)]
pub enum NavEntry {
    Local(Html, Route),
    External(Html, &'static str),
}

#[styled_component(Navigation)]
pub fn navigation() -> Html {
    let theme = use_theme();

    let separator = html! { {" "} };
    let nav_links = [
        NavEntry::Local(
            html! {<><i class="i-profile" />{" Home"}</>},
            Route::Home,
        ),
        NavEntry::External(
            html! {<><i class="i-pencil" />{" Blog"}</>},
            "https://spencer.imbleau.com/blog/",
        ),
        NavEntry::Local(
            html! {<><i class="i-info" />{" Résumé"}</>},
            Route::Resume,
        ),
        NavEntry::Local(
            html! {<><Icon mask={IconMask::Calendar} fill={theme.ac1} />{" Contributions"}</>},
            Route::Contributions,
        ),
        NavEntry::Local(
            html! {<><i class="i-donate" />{" Sponsor"}</>},
            Route::Sponsor,
        ),
        NavEntry::Local(
            html! {<><i class="i-idcard" />{" Contact"}</>},
            Route::Contact,
        ),
    ];

    html! {
        <nav>
            <ul>
            {
                nav_links.iter().map(|n: &NavEntry| {
                    match n {
                        NavEntry::Local(display, route) => html!{
                            <li>
                                <Link<Route> to={ *route }>
                                    { display.clone() }
                                </Link<Route>>
                            </li>
                        },
                        NavEntry::External(display, url) => html!{
                            <li>
                                <a href={ *url }>
                                    { display.clone() }
                                </a>
                            </li>
                        },
                    }
                })
                .intersperse(separator)
                .collect::<Html>()
            }
            </ul>
        </nav>
    }
}
