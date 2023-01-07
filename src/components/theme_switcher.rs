use crate::components::{IconMask, TapTarget};
use crate::style::themes::BrandChoice;
use themer::prelude::*;
use yew::prelude::*;

#[function_component(ThemeSwitcher)]
pub fn theme_switcher() -> Html {
    let theme = use_theme::<BrandChoice>();

    let onclick = {
        let theme = theme.clone();
        let other = match theme.kind() {
            BrandChoice::Light => BrandChoice::Dark,
            BrandChoice::Dark => BrandChoice::Light,
        };
        Callback::from(move |_| {
            theme.set(other);
            _ = BrowserPreference::save(other);
        })
    };

    html! {
        <TapTarget
            mask={match theme.kind() {
                BrandChoice::Light => IconMask::Moon,
                BrandChoice::Dark => IconMask::Brightness
            }}
            { onclick }
        />
    }
}
