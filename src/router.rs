use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{
    contact::ContactPage, cv::CvPage, home::HomePage, not_found::NotFoundPage,
    projects::ProjectsPage, research::ResearchPage, resume::ResumePage,
    sponsor::SponsorPage,
};

#[derive(Routable, Debug, Clone, Copy, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/contact")]
    Contact,
    #[at("/resume")]
    Resume,
    #[at("/cv")]
    CirriculumVitae,
    #[at("/research")]
    Research,
    #[at("/projects")]
    Projects,
    #[at("/sponsor")]
    Sponsor,
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
        Route::Research => html! {<ResearchPage />},
        Route::Projects => html! {<ProjectsPage />},
        Route::Sponsor => html! {<SponsorPage />},
        Route::NotFound => html! {<NotFoundPage />},
    }
}
