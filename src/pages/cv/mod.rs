use crate::pages::construction::ConstructionPage;
use yew::prelude::*;

#[function_component(CvPage)]
pub fn cv_page() -> Html {
    html! { <ConstructionPage message="You're on the CV page" /> }
}
