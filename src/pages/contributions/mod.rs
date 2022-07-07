use crate::pages::construction::ConstructionPage;
use yew::prelude::*;

#[function_component(ContributionsPage)]
pub fn contributions_page() -> Html {
    html! { <ConstructionPage message="You're on the Contributions page" /> }
}
