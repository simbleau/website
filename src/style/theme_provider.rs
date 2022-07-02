use stylist::yew::styled_component;
use yew::prelude::*;

use crate::style::{Theme, ThemeChoice, ThemeContext};

#[derive(Debug, PartialEq, Properties)]
pub struct ThemeProviderProps {
    pub children: Children,
}

#[styled_component(ThemeProvider)]
pub fn theme_provider(props: &ThemeProviderProps) -> Html {
    // Get theme preference from the browser
    let theme_preferemce = Theme::get_preference();
    let theme_state = use_state(move || theme_preferemce);
    let theme_ctx = ThemeContext::new(theme_state);

    html! {
        <ContextProvider<ThemeContext> context={theme_ctx}>
            {props.children.clone()}
        </ContextProvider<ThemeContext>>
    }
}
