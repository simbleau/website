use crate::pages::construction::ConstructionPage;
use yew::prelude::*;

#[function_component(ServerErrorPage)]
pub fn view() -> Html {
    html! { <ConstructionPage title="Error" message="This is embarassing." /> }
}
