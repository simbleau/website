use yew::prelude::*;

use crate::components::TapTarget;
use crate::style::icons::IconMask;
use crate::style::themes::{use_theme, ThemeChoice};

#[function_component(ThemeSwitcher)]
pub fn theme_switcher() -> Html {
    let theme = use_theme();

    let onclick = Callback::from({
        let theme = theme.clone();
        let other = match theme.kind() {
            ThemeChoice::Light => ThemeChoice::Dark,
            ThemeChoice::Dark => ThemeChoice::Light,
        };
        move |_| {
            theme.set(other);
        }
    });

    html! {
        <TapTarget
            mask={match theme.kind() {
                ThemeChoice::Light => IconMask::Moon,
                ThemeChoice::Dark => IconMask::Brightness
            }}
            { onclick }
        />
    }
}
