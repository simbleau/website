use stylist::yew::styled_component;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::{
    router::Route,
    style::{
        icons::{Icon, IconMask},
        use_theme,
    },
};

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
        NavEntry::Local(html! {<>{"Home"}</>}, Route::Home),
        NavEntry::External(
            html! {<>{"Blog"}<Icon mask={IconMask::ArrowUpRightFromSquare} fill={theme.fg1} /></>},
            "https://spencer.imbleau.com/blog/",
        ),
        NavEntry::Local(html! {<>{"Résumé"}</>}, Route::Resume),
        NavEntry::Local(html! {<>{"Contributions"}</>}, Route::Contributions),
        NavEntry::Local(html! {<>{"Sponsor"}</>}, Route::Sponsor),
        NavEntry::Local(html! {<>{"Contact"}</>}, Route::Contact),
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
