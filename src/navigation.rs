use stylist::yew::styled_component;
use yew::prelude::*;

use crate::{
    components::{NavItem, NavLink},
    router::Route,
};

#[styled_component(Navigation)]
pub fn navigation() -> Html {
    let separator = html! { {" "}};
    let nav_links = [
        NavItem::Local(html! {<>{"Home"}</>}, Route::Home),
        NavItem::External(
            html! {<>{ "Blog" }</>},
            "https://simbleau.github.io/blog/",
        ),
        NavItem::Local(html! {<>{"Résumé"}</>}, Route::Resume),
        NavItem::Local(html! {<>{"Contributions"}</>}, Route::Contributions),
        NavItem::Local(html! {<>{"Sponsor"}</>}, Route::Sponsor),
        NavItem::Local(html! {<>{"Contact"}</>}, Route::Contact),
    ];

    html! {
        <nav>
            <ul>
            {
                nav_links.into_iter().map(|n| {
                    html!{
                        <li>
                            <NavLink nav={n} />
                        </li>
                    }
                })
                .intersperse(separator)
                .collect::<Html>()
            }
            </ul>
        </nav>
    }
}
