use accessible_ui::prelude::*;
use stylist::yew::Global;
use themer::prelude::*;
use website::footer::Footer;
use website::header::Header;
use website::router::{self, Route};
use website::style::global::use_global_css;
use website::style::themes::ThemeChoice;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Root)]
pub fn root() -> Html {
    let stored_theme = match BrowserPreference::load::<ThemeChoice>() {
        Some(pref) => pref,
        None => ThemeChoice::default(),
    };

    html! {
        <ThemeProvider<ThemeChoice> theme={stored_theme}>
            <App />
        </ThemeProvider<ThemeChoice>>
    }
}

#[function_component(App)]
fn app() -> Html {
    // Apply global CSS
    let global_css = use_global_css();
    let theme = use_theme::<ThemeChoice>();

    let accessible_ui = AuiSpec {
        color: theme.color,
        background_color: theme.background_color,
        link: theme.link,
        link_hover: theme.link_hover,
        header_color: theme.header_color,
    };

    html! {
        <ContextProvider<AuiSpec> context={ accessible_ui } >
            <BrowserRouter>
                <Global css={global_css} />
                <main>
                    <Header />
                    <div id="content">
                        <Switch<Route> render={router::switch} />
                    </div>
                    <Footer />
                </main>
            </BrowserRouter>
        </ContextProvider<AuiSpec>>
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
