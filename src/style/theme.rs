use crate::style::Color;
use crate::style::ThemeChoice;

#[derive(Debug, Clone)]
pub struct Theme {
    // Foreground color 1
    pub fg1: Color,
    // Foreground color 2
    pub fg2: Color,
    // Background color 1
    pub bg1: Color,
    // Background color 2
    pub bg2: Color,
    // Accent color 1
    pub ac1: Color,
    // Accent color 2
    pub ac2: Color,
}

impl Theme {
    /// Get the browser's theme preference through a media query.
    pub fn get_preference() -> ThemeChoice {
        // TODO: Check local storage for a preference

        // No local storage preference found, query the browser for a preference
        let window = web_sys::window().unwrap();
        match window
            .match_media("(prefers-color-scheme: dark)")
            .unwrap()
            .unwrap()
            .matches()
        {
            true => {
                // Browser prefers dark theme
                ThemeChoice::Dark
            }
            false => {
                // Browser prefers dark theme
                ThemeChoice::Light
            }
        }
    }
}
