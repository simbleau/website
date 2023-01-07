use crate::{
    components::{Destination, NavLink},
    router::Route,
};
use stylist::yew::styled_component;
use yew::prelude::*;

#[styled_component(Navigation)]
pub fn navigation() -> Html {
    // If updating the links, update the sitemap!
    let nav_links: [(Destination, Html); 3] = [
        (Destination::Internal(Route::Home), html!({ "Home" })),
        (
            Destination::External("https://simbleau.github.io/blog/"),
            html! {{ "Blog" }},
        ),
        (Destination::Internal(Route::Resume), html!({ "Résumé" })),
    ];
    let separator = html! {{" "}};

    let links_css = css! {
        a,a:hover {
            text-decoration:none;
        }
    };

    html! {
        <nav>
            <ul class={links_css}>
            {
                nav_links.into_iter().map(|(domain, display)| {
                    html!{
                        <li>
                            <NavLink to={domain}>
                                { display }
                            </NavLink>
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
