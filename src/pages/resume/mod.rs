use crate::pages::construction::ConstructionPage;
use yew::prelude::*;

#[function_component(ResumePage)]
pub fn resume_page() -> Html {
    html! { <ConstructionPage message="You're on the Résumé page" /> }
}
