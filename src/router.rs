use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{
    contact::ContactPage, contributions::ContributionsPage, cv::CvPage,
    home::HomePage, not_found::NotFoundPage, resume::ResumePage,
    server_error::ServerErrorPage, sponsor::SponsorPage,
};

#[derive(Routable, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/contact")]
    Contact,
    #[at("/resume")]
    Resume,
    #[at("/cv")]
    CirriculumVitae,
    #[at("/contributions")]
    Contributions,
    #[at("/sponsor")]
    Sponsor,
    #[at("/error")]
    Error,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: &Route) -> Html {
    match route {
        Route::Home => html! {<HomePage />},
        Route::Contact => html! {<ContactPage />},
        Route::Resume => html! {<ResumePage />},
        Route::CirriculumVitae => html! {<CvPage />},
        Route::Contributions => html! {<ContributionsPage />},
        Route::Sponsor => html! {<SponsorPage />},
        Route::Error => html! {<ServerErrorPage />},
        Route::NotFound => html! {<NotFoundPage />},
    }
}
