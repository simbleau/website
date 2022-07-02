use std::ops::Deref;
use yew::prelude::*;

use crate::themes::Theme;
use crate::themes::ThemeChoice;

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
