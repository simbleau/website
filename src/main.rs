use yew::prelude::*;
mod pages;
use pages::construction::ConstructionPage;

const UNDER_CONSTRUCTION: bool = true;

#[function_component(App)]
fn app() -> Html {
    html! {
        if UNDER_CONSTRUCTION {
            <ConstructionPage message={"You shall not pass!"} end={"July 2022".to_string()} />
        } else {
            // TODO: Replace with real website.
            <h3>{ "Not under construction." }</h3>
        }
    }
}

fn main() {
    gloo_console::log!("Hello from Rust + WASM!");
    yew::start_app::<App>();
}
