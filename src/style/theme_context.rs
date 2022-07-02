use gloo_console::log;
use std::ops::Deref;
use wasm_bindgen::UnwrapThrowExt;
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

    pub fn set(&self, kind: ThemeChoice) {
        self.inner.set(kind)
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
