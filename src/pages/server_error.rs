use crate::components::Construction;
use yew::prelude::*;

#[function_component]
pub fn ServerErrorPage() -> Html {
    html! { <Construction title="Error" message="This is embarassing." /> }
}
