use crate::pages::construction::ConstructionPage;
use yew::prelude::*;

#[function_component(ContactPage)]
pub fn contact_page() -> Html {
    html! { <ConstructionPage message="You're on the contact page" /> }
}
