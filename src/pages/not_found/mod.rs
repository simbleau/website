use crate::pages::construction::ConstructionPage;
use yew::prelude::*;

#[function_component(NotFoundPage)]
pub fn not_found_page() -> Html {
    html! { <ConstructionPage title="Yikes" message="How did you get here?" /> }
}
