use crate::components::Construction;
use yew::prelude::*;

#[function_component(ServerErrorPage)]
pub fn view() -> Html {
    html! { <Construction title="Error" message="This is embarassing." /> }
}
