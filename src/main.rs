mod components;
mod footer;
mod header;
mod navigation;
mod pages;
mod router;
mod style;
mod util;

use crate::{
    header::Header,
    router::Route,
    style::{global::use_global_css, themes::ThemeChoice},
};
use log::info;
use stylist::yew::Global;
use themer::{browser::BrowserPreference, yew::ThemeProvider};
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Root)]
pub fn root() -> Html {
    let stored_theme = match BrowserPreference::load::<ThemeChoice>() {
        Some(pref) => pref,
        None => ThemeChoice::default(),
    };
    info!("Theme: {stored_theme:?}");

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
    html! {
        <BrowserRouter>
            <Global css={global_css} />
            <main>
                <Header />
                <div id="content">
                    <Switch<Route> render={router::switch} />
                </div>
            </main>
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
