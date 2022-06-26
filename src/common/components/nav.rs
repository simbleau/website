use crate::router::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Copy)]
pub enum NavEntry {
    Local(&'static str, Route),
    External(&'static str, &'static str),
}

const NAV_LINKS: [NavEntry; 3] = [
    NavEntry::Local("Home", Route::Home),
    NavEntry::External("Blog", "https://spencer.imbleau.com/blog/"),
    NavEntry::Local("Contact", Route::Contact),
];

#[function_component(Navigation)]
pub fn navigation() -> Html {
    html! {
        <nav>
            {
                NAV_LINKS.iter().map(|n: &NavEntry| {
                    match n {
                        NavEntry::Local(display, route) => html!{
                            <Link<Route> to={*route}>{display}</Link<Route>>
                        },
                        NavEntry::External(display, url) => html!{
                            <a href={*url}>{display}</a>
                        },
                    }
                })
                .intersperse(html!{" | "})
                .collect::<Html>()
            }
        </nav>
    }
}
