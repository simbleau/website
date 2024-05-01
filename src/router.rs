use crate::page_components::*;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Routable, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Route {
    #[at("/error")]
    Error,
    #[not_found]
    #[at("/404")]
    NotFound,

    #[at("/")]
    Home,
    #[at("/resume")]
    Resume,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Error => html! {<ServerErrorPage />},
        Route::NotFound => html! {<NotFoundPage />},

        Route::Home => html! {<HomePage />},
        Route::Resume => html! {<ResumePage />},
    }
}
