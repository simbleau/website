use crate::{style::themes::ThemeSpec, util::lighten};
use hex_color::HexColor;
use once_cell::sync::Lazy;

pub static LIGHT_THEME: Lazy<ThemeSpec> = Lazy::new(|| ThemeSpec {
    color: HexColor::from_u24(0x212529),
    background_color: HexColor::from_u24(0xf8f9fa),
    link: HexColor::from_u24(0x0033dd),
    link_hover: HexColor::from_u24(0x4C7FFF),
    header_color: HexColor::from_u24(0x0033dd),
    header_font: "Roboto",
    body_font: "Roboto",
    mono_font: "Roboto",
    fs: "1rem",
    fsm: "1.04rem",
    fst: "1.08rem",
    fsd: "1.12rem",
    fw: "400",
    fwh: "700",
});
