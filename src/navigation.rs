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
        (NavDestination::Internal(Route::Home), html!({ "home" })),
        (NavDestination::Internal(Route::Resume), html!({ "résumé" })),
        (
            NavDestination::External(AttrValue::Static(
                "https://simbleau.github.io/",
            )),
            html! {{ "blog" }},
        ),
    ];

    let links_css = css! {
        padding: 0;
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
