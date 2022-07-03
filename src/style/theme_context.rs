use gloo_storage::Storage;
use std::ops::Deref;
use yew::prelude::*;

use crate::style::Theme;
use crate::style::ThemeChoice;

#[derive(Debug, Clone)]
pub struct ThemeContext {
    inner: UseStateHandle<ThemeChoice>,
}

impl ThemeContext {
    pub fn new(inner: UseStateHandle<ThemeChoice>) -> Self {
        Self { inner }
    }

    pub fn set(&self, choice: ThemeChoice) {
        self.inner.set(choice);
        // Try to save in local storage
        if gloo_storage::LocalStorage::set("theme", choice).is_ok() {
            log::info!("Theme preference saved: {:?}", choice);
        };
    }

    pub fn kind(&self) -> ThemeChoice {
        *self.inner
    }
}

impl Deref for ThemeContext {
    type Target = Theme;

    fn deref(&self) -> &Self::Target {
        &(*self.inner.current())
    }
}

impl PartialEq for ThemeContext {
    fn eq(&self, rhs: &Self) -> bool {
        *self.inner == *rhs.inner
    }
}
