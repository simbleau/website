use crate::{
    components::{IconMask, TapTarget},
    hooks::{BrowserPreference, use_theme},
    style::themes::ThemeChoice,
};
use log::info;
use yew::prelude::*;

#[function_component(ThemeSwitcher)]
pub fn view() -> Html {
    let theme = use_theme();
    let onclick = {
        let theme = theme.clone();
        Callback::from(move |_| {
            let other = match theme.kind() {
                ThemeChoice::Light => ThemeChoice::Dark,
                ThemeChoice::Dark => ThemeChoice::Light,
                _ => ThemeChoice::default(),
            };
            info!("Theme set: {other:?}");
            theme.set(other);
            _ = BrowserPreference::save(other);
        })
    };

    html! {
        <TapTarget
            aria_label="Switch Theme"
            mask={match theme.kind() {
                ThemeChoice::Light => IconMask::Moon,
                ThemeChoice::Dark => IconMask::Sun,
                ThemeChoice::Lego => IconMask::Lego,
            }}
            { onclick }
        />
    }
}
