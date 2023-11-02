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
use log::{info, warn};
use stylist::yew::{styled_component, Global};
use themer::{browser::BrowserPreference, yew::ThemeProvider};
use url::Url;
use web_sys::window;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Root)]
pub fn root() -> Html {
    // Get stored theme
    let mut stored_theme = match BrowserPreference::load::<ThemeChoice>() {
        Some(pref) => pref,
        None => ThemeChoice::default(),
    };
    info!("Theme: {stored_theme:?}");

    // Override with theme from query string
    let query_string_theme = window()
        .and_then(|w| w.location().href().ok())
        .and_then(|href| Url::parse(&href).ok())
        .and_then(|url| {
            url.query_pairs()
                .find(|(k, _v)| k == "theme")
                .map(|(_k, v)| v.to_string())
        });
    if let Some(theme) = query_string_theme.as_deref() {
        match theme {
            "lego" => {
                stored_theme = ThemeChoice::Lego;
            }
            theme => {
                warn!("unrecognized theme: {theme}");
            }
        };
    }

    html! {
        <ThemeProvider<ThemeChoice> theme={stored_theme}>
            <App />
        </ThemeProvider<ThemeChoice>>
    }
}

#[styled_component]
fn App() -> Html {
    // Apply global CSS
    let global_css = use_global_css();

    let main_container_css = css! {
        display: flex;
        flex-direction: column;
        margin: 10px;
    };
    let content_container_css = css! {
        align-self: center;
        width: 100%;
        max-width: 1000px;
    };

    html! {
        <BrowserRouter>
            <Global css={global_css.0} />
            <Global css={global_css.1} />
            <main class={main_container_css}>
                <Header />
                <div id="content" align="center" class={content_container_css}>
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
