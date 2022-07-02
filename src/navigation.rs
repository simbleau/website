use stylist::yew::styled_component;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::Route;

#[derive(Clone, Copy)]
pub enum NavEntry {
    Local(fn() -> Html, Route),
    External(fn() -> Html, &'static str),
}

pub const SEPARATOR: fn() -> Html = || html! { {" "} };
pub const NAV_LINKS: [NavEntry; 7] = [
    NavEntry::Local(
        || html! {<><i class="i-profile" />{" Home"}</>},
        Route::Home,
    ),
    NavEntry::External(
        || html! {<><i class="i-pencil" />{" Blog"}</>},
        "https://spencer.imbleau.com/blog/",
    ),
    NavEntry::Local(
        || html! {<><i class="i-info" />{" Résumé"}</>},
        Route::Resume,
    ),
    NavEntry::Local(
        || html! {<><i class="i-bookmark" />{" Research"}</>},
        Route::Research,
    ),
    NavEntry::Local(
        || html! {<><i class="i-bug" />{" Projects"}</>},
        Route::Projects,
    ),
    NavEntry::Local(
        || html! {<><i class="i-donate" />{" Sponsor"}</>},
        Route::Sponsor,
    ),
    NavEntry::Local(
        || html! {<><i class="i-idcard" />{" Contact"}</>},
        Route::Contact,
    ),
];

#[styled_component(Navigation)]
pub fn navigation() -> Html {
    html! {
        <nav>
            <p>
            {
                NAV_LINKS.iter().map(|n: &NavEntry| {
                    match n {
                        NavEntry::Local(display, route) => html!{
                            <Link<Route> to={ *route }>{ display() }</Link<Route>>
                        },
                        NavEntry::External(display, url) => html!{
                            <a href={ *url }>{ display() }</a>
                        },
                    }
                })
                .intersperse(SEPARATOR())
                .collect::<Html>()
            }
            </p>
        </nav>
    }
}
