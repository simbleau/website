use stylist::yew::styled_component;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::Button;
use crate::router::Route;

#[derive(Properties, PartialEq, Eq)]
pub struct ConstructionProps {
    #[prop_or("Under Construction!".to_string())]
    pub title: String,
    #[prop_or("Pardon my dust.".to_string())]
    pub message: String,
    #[prop_or_default]
    pub end: Option<String>,
}

#[styled_component(Construction)]
pub fn view(props: &ConstructionProps) -> Html {
    let nav = use_navigator().unwrap();
    let onclick = Callback::from(move |_| nav.push(&Route::Home));

    html! {
        <div align="center">
            <h1>{ &props.title }</h1>
            <h2>{ &props.message }</h2>
            if let Some(end) = &props.end {
                <p>{ "Expected completion: " } { end }</p>
            }
            <br />
            <Button onclick={onclick}>
                <h2>{"Return"}</h2>
            </Button>
        </div>
    }
}
