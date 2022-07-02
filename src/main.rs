use stylist::css;
use stylist::yew::Global;
use yew::prelude::*;
use yew_router::prelude::*;

use website::footer::{Footer, FOOTER_HEIGHT};
use website::header::Header;
use website::pages::construction::ConstructionPage;
use website::router;
use website::style::{use_theme, ThemeChoice, ThemeProvider};

const UNDER_CONSTRUCTION: bool = false;

#[function_component(Root)]
pub fn root() -> Html {
    html! {
        <ThemeProvider>
            <App />
        </ThemeProvider>
    }
}

#[function_component(App)]
fn app() -> Html {
    let theme = use_theme();

    let main_style = css!(
        r#"
            html, body {
                padding: 0;
                margin: 0;
                position: relative;
                min-height: 100vh;
                background-color: ${bg};
                color: ${fg};
            }
        "#,
        bg = theme.bg1.to_css(),
        fg = theme.fg1.to_css(),
    );

    html! {
        <>
        <Global css={main_style}/>
        if UNDER_CONSTRUCTION {
            <ConstructionPage message={"You shall not pass!"} end={"July 2022".to_string()} />
        } else {
            <BrowserRouter>
                <main>
                    <Header />
                    <div id="content" class={ css!("padding-bottom: ${fh};", fh = FOOTER_HEIGHT) }>
                        <Switch<router::Route> render={Switch::render(router::switch)} />
                    </div>
                    <Footer />
                </main>
            </BrowserRouter>
        }
        </>
    }
}

fn main() {
    gloo_console::log!("Hello from Rust + WASM!");
    yew::start_app::<Root>();
}
