use crate::components::Construction;
use yew::prelude::*;

#[function_component]
pub fn NotFoundPage() -> Html {
    html! { <Construction title="Yikes" message="How did you get here?" /> }
}
