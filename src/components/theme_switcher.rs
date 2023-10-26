use crate::{
    components::{IconMask, TapTarget},
    style::themes::ThemeChoice,
};
use log::info;
use themer::prelude::*;
use yew::prelude::*;

#[function_component(ThemeSwitcher)]
pub fn theme_switcher() -> Html {
    let theme = use_theme::<ThemeChoice>();
    let onclick = {
        let theme = theme.clone();
        Callback::from(move |_| {
            let other = match theme.kind() {
                ThemeChoice::Light => ThemeChoice::Dark,
                ThemeChoice::Dark => ThemeChoice::Light,
            };
            info!("Theme set: {other:?}");
            theme.set(other);
            _ = BrowserPreference::save(other);
        })
    };

    html! {
        <TapTarget
            mask={match theme.kind() {
                ThemeChoice::Light => IconMask::Moon,
                ThemeChoice::Dark => IconMask::Sun
            }}
            { onclick }
        />
    }
}
