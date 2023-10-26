use crate::router::Route;
use stylist::yew::styled_component;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq, Eq)]
pub struct ConstructionProps {
    #[prop_or(AttrValue::Static("Under Construction!"))]
    pub title: AttrValue,
    #[prop_or(AttrValue::Static("Pardon my dust."))]
    pub message: AttrValue,
    #[prop_or_default]
    pub end: Option<String>,
}

#[styled_component]
pub fn Construction(props: &ConstructionProps) -> Html {
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
            <button onclick={onclick}>
                <h2>{"Return"}</h2>
            </button>
        </div>
    }
}
