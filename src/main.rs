use stylist::style;
use website::common::components::footer::Footer;
use website::common::components::footer::FOOTER_HEIGHT;
use website::common::components::header::Header;
use website::pages::construction::ConstructionPage;
use website::router;
use yew::prelude::*;
use yew_router::prelude::*;

const UNDER_CONSTRUCTION: bool = false;

#[function_component(App)]
fn app() -> Html {
    let main_style = style!(
        r#"
            position: relative;
            min-height: 100vh;
        "#,
    )
    .unwrap();

    let content_style = style!(
        r#"
            padding-bottom: ${footer_height};
        "#,
        footer_height = FOOTER_HEIGHT,
    )
    .unwrap();

    html! {
        if UNDER_CONSTRUCTION {
            <ConstructionPage message={"You shall not pass!"} end={"July 2022".to_string()} />
        } else {
            <BrowserRouter>
                <div id="main" class={main_style}>
                    <div id="content" class={content_style}>
                        <Header />
                        <Switch<router::Route> render={Switch::render(router::switch)} />
                    </div>
                    <Footer />
                </div>
            </BrowserRouter>
        }
    }
}

fn main() {
    gloo_console::log!("Hello from Rust + WASM!");
    yew::start_app::<App>();
}
