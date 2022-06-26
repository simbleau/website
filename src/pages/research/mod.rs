use crate::pages::construction::ConstructionPage;
use yew::prelude::*;

#[function_component(ResearchPage)]
pub fn research_page() -> Html {
    html! { <ConstructionPage message="You're on the Research page" /> }
}
