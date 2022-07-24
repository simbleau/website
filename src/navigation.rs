use stylist::yew::styled_component;
use yew::prelude::*;

use crate::{
    components::{NavLink, Url},
    router::Route,
};

#[styled_component(Navigation)]
pub fn navigation() -> Html {
    // If updating the links, update the sitemap!
    let nav_links: [(Url, Html); 6] = [
        (Url::Local(Route::Home), html! {{"Home"}}),
        (
            Url::External("https://simbleau.github.io/blog/"),
            html! {{ "Blog" }},
        ),
        (Url::Local(Route::Resume), html! {{"Résumé"}}),
        (Url::Local(Route::Contributions), html! {{"Contributions"}}),
        (Url::Local(Route::Sponsor), html! {{"Sponsor"}}),
        (Url::Local(Route::Contact), html! {{"Contact"}}),
    ];
    let separator = html! {{" "}};

    html! {
        <nav>
            <ul>
            {
                nav_links.into_iter().map(|(domain, display)| {
                    html!{
                        <li>
                            <NavLink domain={domain} display={display} />
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
