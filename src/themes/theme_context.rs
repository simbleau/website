use once_cell::sync::Lazy;
use std::ops::Deref;
use stylist::yew::styled_component;
use yew::html::ImplicitClone;
use yew::prelude::*;

use crate::themes::Theme;
use crate::themes::ThemeKind;

#[derive(Debug, Clone)]
pub struct ThemeContext {
    inner: UseStateHandle<ThemeKind>,
}

impl ThemeContext {
    pub fn new(inner: UseStateHandle<ThemeKind>) -> Self {
        Self { inner }
    }

    pub fn set(&self, kind: ThemeKind) {
        self.inner.set(kind)
    }

    pub fn kind(&self) -> ThemeKind {
        (*self.inner).clone()
    }
}

impl Deref for ThemeContext {
    type Target = Theme;

    fn deref(&self) -> &Self::Target {
        &*self.inner.current()
    }
}

impl PartialEq for ThemeContext {
    fn eq(&self, rhs: &Self) -> bool {
        *self.inner == *rhs.inner
    }
}
