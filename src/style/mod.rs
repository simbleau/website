pub mod global;

mod theme_provider;
pub use theme_provider::ThemeProvider;

mod theme_context;
pub use theme_context::ThemeContext;

mod theme_choice;
pub use theme_choice::ThemeChoice;

mod icon;
pub use icon::Icon;
pub use icon::IconMask;

mod theme;
pub use theme::Theme;
pub mod themes;

pub mod color;

use yew::use_context;
pub fn use_theme() -> ThemeContext {
    use_context::<ThemeContext>().unwrap()
}
