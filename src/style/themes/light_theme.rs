use crate::style::themes::ThemeSpec;
use hex_color::HexColor;
use once_cell::sync::Lazy;

pub static LIGHT_THEME: Lazy<ThemeSpec> = Lazy::new(|| ThemeSpec {
    color: HexColor::from_u24(0x212529),
    background_color: HexColor::from_u24(0xf8f9fa),
    link: HexColor::from_u24(0x0033dd),
    link_hover: HexColor::from_u24(0x4C7FFF),
    body_font: "Roboto",
    mono_font: "Roboto",
    fsm: "1.2rem",
    fst: "1.4rem",
    fsd: "1.6rem",
    fw: "400",
    fwh: "700",
});
