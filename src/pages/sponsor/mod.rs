use crate::pages::construction::ConstructionPage;
use yew::prelude::*;

#[function_component(SponsorPage)]
pub fn sponsor_page() -> Html {
    html! { <ConstructionPage message="You're on the Sponsor page" /> }
}
