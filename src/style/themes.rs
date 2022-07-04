use once_cell::sync::Lazy;

use crate::style::colors::{BLACK, WHITE};
use crate::style::Color;
use crate::style::Theme;

pub static LIGHT_THEME: Lazy<Theme> = Lazy::new(|| Theme {
    fg1: BLACK,
    fg2: BLACK,
    bg1: WHITE,
    bg2: WHITE,
    ac1: BLACK,
    ac2: BLACK,
});

pub static DARK_THEME: Lazy<Theme> = Lazy::new(|| Theme {
    fg1: WHITE,
    fg2: WHITE,
    bg1: BLACK,
    bg2: BLACK,
    ac1: WHITE,
    ac2: WHITE,
});
