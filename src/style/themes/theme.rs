use gloo_storage::Storage;

use super::ThemeChoice;
use crate::style::colors::Color;

#[derive(Debug, Clone)]
pub struct Theme {
    /// Foreground color 1
    pub fg1: Color,
    /// Foreground color 2
    pub fg2: Color,
    /// Background color 1
    pub bg1: Color,
    /// Background color 2
    pub bg2: Color,
    /// Accent color 1
    pub ac1: Color,
    /// Accent color 2
    pub ac2: Color,

    /// Font size - default
    pub fs: &'static str,
    /// Font size - mobile
    pub fsm: &'static str,
    /// Font size - tablet
    pub fst: &'static str,
    /// Font size - desktop
    pub fsd: &'static str,

    /// Font width - text
    pub fw: &'static str,
    /// Font width - headers
    pub fwh: &'static str,
}

impl Theme {
    /// Get the browser's theme preference through a med ia query.
    pub fn get_preference() -> ThemeChoice {
        match gloo_storage::LocalStorage::get("theme") {
            Ok(preference) => {
                // Get saved theme from local storage
                preference
            }
            Err(_) => {
                // No local storage preference found, query the browser for a
                // preference
                match web_sys::window()
                    .and_then(|w| {
                        w.match_media("(prefers-color-scheme: dark)").ok()
                    })
                    .flatten()
                    .map(|m| m.matches())
                {
                    // Browser prefers dark theme
                    Some(true) => ThemeChoice::Dark,
                    // Browser prefers light theme
                    Some(false) => ThemeChoice::Light,
                    // Browser was not queryable
                    None => ThemeChoice::default(),
                }
            }
        }
    }
}
