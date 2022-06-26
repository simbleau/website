use crate::pages::construction::ConstructionPage;
use yew::prelude::*;

#[function_component(HomePage)]
pub fn home_page() -> Html {
    html! { <ConstructionPage /> }
}
