use crate::components::Construction;
use stylist::yew::styled_component;
use yew::prelude::*;

#[styled_component(NotFoundPage)]
pub fn view() -> Html {
    html! { <Construction title="Yikes" message="How did you get here?" /> }
}
