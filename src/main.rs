use stylist::yew::Global;
use themer::prelude::*;
use website::footer::Footer;
use website::header::Header;
use website::pages::ConstructionPage;
use website::router::{self, Route};
use website::style::global::use_global_css;
use website::style::themes::BrandChoice;
use yew::prelude::*;
use yew_router::prelude::*;

const UNDER_CONSTRUCTION: bool = false;

#[function_component(Root)]
pub fn root() -> Html {
    let stored_theme = match BrowserPreference::load::<BrandChoice>() {
        Some(pref) => pref,
        None => BrandChoice::default(),
    };

    html! {
        <ThemeProvider<BrandChoice> theme={stored_theme}>
            <App />
        </ThemeProvider<BrandChoice>>
    }
}

#[function_component(App)]
fn app() -> Html {
    // Apply global CSS
    let global_css = use_global_css();

    html! {
        <BrowserRouter>
            <Global css={global_css} />
            if UNDER_CONSTRUCTION {
                <ConstructionPage message={"You shall not pass!"} end={"July 2022".to_string()} />
            } else {
                    <main>
                        <Header />
                        <div id="content">
                            <Switch<Route> render={router::switch} />
                        </div>
                        <Footer />
                    </main>
            }
        </BrowserRouter>
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

    yew::Renderer::<Root>::new().render();
}
