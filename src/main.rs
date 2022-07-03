use stylist::css;
use stylist::yew::Global;
use yew::prelude::*;
use yew_router::prelude::*;

use website::footer::{Footer, FOOTER_HEIGHT};
use website::header::Header;
use website::pages::construction::ConstructionPage;
use website::router;
use website::style::{use_theme, ThemeProvider};

// A smaller allocator to save some size on the WASM bundle
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

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
                /* General styling */
                padding: 0;
                margin: 0;
                position: relative;
                min-height: 100vh;
                scroll-behavior: smooth;

                /* Apply theme */
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
    #[cfg(debug_assertions)]
    {
        // Initialize log and panics to forward to browser log if debugging
        console_log::init_with_level(log::Level::Trace)
            .expect("Failed to initialise Log!");
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    }

    yew::start_app::<Root>();
}
