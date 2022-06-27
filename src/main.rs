use stylist::{css, style};
use website::router;
use yew::prelude::*;
use yew_router::prelude::*;

use website::footer::{Footer, FOOTER_HEIGHT};
use website::navigation::Navigation;
use website::pages::construction::ConstructionPage;

const UNDER_CONSTRUCTION: bool = false;

#[function_component(App)]
fn app() -> Html {
    // TODO: Theme
    let main_style = style!(
        r#"
            position: relative;
            min-height: 100vh;
        "#,
    )
    .unwrap();

    html! {
        if UNDER_CONSTRUCTION {
            <ConstructionPage message={"You shall not pass!"} end={"July 2022".to_string()} />
        } else {
            <BrowserRouter>
                <main class={ main_style }>
                    <Navigation />
                    <div id="content" class={ css!("padding-bottom: ${fh};", fh = FOOTER_HEIGHT) }>
                        <Switch<router::Route> render={Switch::render(router::switch)} />
                    </div>
                    <Footer />
                </main>
            </BrowserRouter>
        }
    }
}

fn main() {
    gloo_console::log!("Hello from Rust + WASM!");
    yew::start_app::<App>();
}
