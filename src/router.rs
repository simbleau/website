use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{
    home::HomePage, not_found::NotFoundPage, resume::ResumePage,
    server_error::ServerErrorPage,
};

#[derive(Routable, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/resume")]
    Resume,
    #[at("/error")]
    Error,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: &Route) -> Html {
    match route {
        Route::Home => html! {<HomePage />},
        Route::Resume => html! {<ResumePage />},
        Route::Error => html! {<ServerErrorPage />},
        Route::NotFound => html! {<NotFoundPage />},
    }
}
