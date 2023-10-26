use crate::style::themes::ThemeSpec;
use once_cell::sync::Lazy;

pub static LIGHT_THEME: Lazy<ThemeSpec> = Lazy::new(|| ThemeSpec {
    color: "#212529",
    background_color: "#f8f9fa",
    link: "#0033dd",
    link_hover: "#0013bd",
    header_color: "0x0033dd",
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
