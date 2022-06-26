use crate::pages::construction::ConstructionPage;
use yew::prelude::*;

#[function_component(NotFoundPage)]
pub fn not_found_page() -> Html {
    html! { <ConstructionPage title="404" message="You shall not pass!" /> }
}
