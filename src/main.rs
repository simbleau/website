use stylist::style;
use website::common::components::footer::Footer;
use website::common::components::header::Header;
use website::pages::construction::ConstructionPage;
use website::router;
use yew::prelude::*;
use yew_router::prelude::*;

const UNDER_CONSTRUCTION: bool = false;

#[function_component(App)]
fn app() -> Html {
    let style = style!(
        r#"
            position: relative;
            min-height: 100vh;

            #content-wrap {
            padding-bottom: 50px;    /* Footer height */
            }
        "#
    )
    .unwrap();

    html! {
        if UNDER_CONSTRUCTION {
            <ConstructionPage message={"You shall not pass!"} end={"July 2022".to_string()} />
        } else {
            <BrowserRouter>
            <div id="page-container" class={style}>
                <div id="content-wrap">
                    <Header />
                    <main>
                        <Switch<router::Route> render={Switch::render(router::switch)} />
                    </main>
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
