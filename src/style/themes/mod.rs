pub fn use_theme() -> ThemeContext {
    yew::use_context::<ThemeContext>().unwrap()
}

mod theme_choice;
pub use theme_choice::ThemeChoice;

mod theme_context;
pub use theme_context::ThemeContext;

mod theme_provider;
pub use theme_provider::ThemeProvider;

mod theme;
pub use theme::Theme;

mod light_theme;
pub use light_theme::LIGHT_THEME;

mod dark_theme;
pub use dark_theme::DARK_THEME;
