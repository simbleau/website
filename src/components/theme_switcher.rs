use yew::prelude::*;

use crate::style::{use_theme, ThemeChoice};

#[function_component(ThemeSwitcher)]
pub fn theme_switcher() -> Html {
    let theme = use_theme();

    let other_theme = match theme.kind() {
        ThemeChoice::Light => ThemeChoice::Dark,
        ThemeChoice::Dark => ThemeChoice::Light,
    };
    let other_icon: fn() -> Html = match theme.kind() {
        ThemeChoice::Light => || html!( <i class="i-moon" /> ),
        ThemeChoice::Dark => || html!( <i class="i-sun" /> ),
    };
    let switch_theme = Callback::from(move |_| theme.set(other_theme));

    html! {
        <button onclick={switch_theme}>{ other_icon() }</button>
    }
}
