mod theme_context;
pub use theme_context::ThemeContext;

mod theme_kind;
pub use theme_kind::ThemeKind;

mod theme_provider;
pub use theme_provider::ThemeProvider;

mod theme;
pub use theme::Theme;

mod color;
pub use color::Color;

use yew::use_context;
pub fn use_theme() -> ThemeContext {
    use_context::<ThemeContext>().expect("Called from outside ThemeContext")
}
