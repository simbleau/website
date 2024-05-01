use crate::style::themes::Theme;
use crate::style::themes::ThemeChoice;
use gloo_storage::errors::StorageError;
use gloo_storage::Storage;
use serde::Deserialize;
use serde::Serialize;
use std::ops::Deref;
use yew::prelude::*;
use yew::use_context;

#[yew::hook]
pub fn use_theme() -> ThemeContext {
    use_context::<ThemeContext>().expect("no theme provider")
}

#[derive(Debug, Clone)]
pub struct ThemeContext {
    inner: UseStateHandle<ThemeChoice>,
}

impl ThemeContext {
    pub fn new(inner: UseStateHandle<ThemeChoice>) -> Self {
        Self { inner }
    }

    pub fn set(&self, key: ThemeChoice) {
        self.inner.set(key);
    }

    pub fn kind(&self) -> ThemeChoice {
        *self.inner
    }
}

impl Deref for ThemeContext {
    type Target = Theme;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl PartialEq for ThemeContext {
    fn eq(&self, rhs: &Self) -> bool {
        *self.inner == *rhs.inner
    }
}

#[derive(Debug, PartialEq, Properties)]
pub struct ThemeProviderProps {
    pub theme: ThemeChoice,
    pub children: Children,
}

#[function_component(ThemeProvider)]
pub fn view(props: &ThemeProviderProps) -> Html {
    let themekey = props.theme;
    let theme_state = use_state(move || themekey);
    let theme_ctx = ThemeContext::new(theme_state);

    html! {
        <ContextProvider<ThemeContext> context={ theme_ctx }>
            {props.children.clone()}
        </ContextProvider<ThemeContext>>
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BrowserPreference {
    Dark,
    Light,
}

impl BrowserPreference {
    pub fn get() -> Option<Self> {
        // query the browser for preference
        match web_sys::window()
            .and_then(|w| w.match_media("(prefers-color-scheme: dark)").ok())
            .flatten()
            .map(|m| m.matches())
        {
            // Browser prefers dark theme
            Some(true) => Some(BrowserPreference::Dark),
            // Browser prefers light theme
            Some(false) => Some(BrowserPreference::Light),
            // Browser was not queryable
            None => None,
        }
    }

    pub fn load() -> Option<ThemeChoice> {
        gloo_storage::LocalStorage::get("theme").ok()
    }

    pub fn save(key: ThemeChoice) -> Result<(), StorageError> {
        gloo_storage::LocalStorage::set("theme", key)
    }
}
