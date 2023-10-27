use crate::{style::themes::ThemeSpec, util::lighten};
use hex_color::HexColor;
use once_cell::sync::Lazy;

pub static DARK_THEME: Lazy<ThemeSpec> = Lazy::new(|| ThemeSpec {
    color: HexColor::from_u24(0xcccccc),
    background_color: HexColor::from_u24(0x222222),
    link: HexColor::from_u24(0x66bbff),
    link_hover: { lighten(&HexColor::from_u24(0x66bbff), 1.2) },
    body_font: "Roboto",
    mono_font: "Roboto",
    fs: "1rem",
    fsm: "1.04rem",
    fst: "1.08rem",
    fsd: "1.12rem",
    fw: "400",
    fwh: "700",
});
