pub mod global;

mod theme_provider;
pub use theme_provider::ThemeProvider;

mod theme_context;
pub use theme_context::ThemeContext;

mod theme_choice;
pub use theme_choice::ThemeChoice;

pub mod icons;

mod theme;
pub use theme::Theme;
pub mod themes;

pub mod colors;

use yew::use_context;
pub fn use_theme() -> ThemeContext {
    use_context::<ThemeContext>().unwrap()
}
