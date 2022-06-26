use website::common::components::nav::Navigation;
use website::pages::construction::ConstructionPage;
use website::router;
use yew::prelude::*;
use yew_router::prelude::*;

const UNDER_CONSTRUCTION: bool = false;

#[function_component(App)]
fn app() -> Html {
    html! {
        if UNDER_CONSTRUCTION {
            <ConstructionPage message={"You shall not pass!"} end={"July 2022".to_string()} />
        } else {
            <BrowserRouter>
                <Navigation />
                <Switch<router::Route> render={Switch::render(router::switch)} />
            </BrowserRouter>
            // TODO: Footer
        }
    }
}

fn main() {
    gloo_console::log!("Hello from Rust + WASM!");
    yew::start_app::<App>();
}
