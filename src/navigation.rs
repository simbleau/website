use crate::{
    components::{NavDestination, NavLink},
    router::Route,
};
use stylist::yew::styled_component;
use yew::prelude::*;

#[styled_component(Navigation)]
pub fn navigation() -> Html {
    // If updating the links, update the sitemap!
    let nav_links: [(NavDestination, Html); 3] = [
        (NavDestination::Internal(Route::Home), html!({ "Home" })),
        (
            NavDestination::External(AttrValue::Static(
                "https://simbleau.github.io/blog/",
            )),
            html! {{ "Blog" }},
        ),
        (NavDestination::Internal(Route::Resume), html!({ "Résumé" })),
    ];

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
                .collect::<Html>()
            }
            </ul>
        </nav>
    }
}
