use crate::pages::construction::ConstructionPage;
use yew::prelude::*;

#[function_component(ProjectsPage)]
pub fn projects_page() -> Html {
    html! { <ConstructionPage message="You're on the Projects page" /> }
}