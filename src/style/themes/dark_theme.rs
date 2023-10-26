use crate::style::themes::ThemeSpec;
use once_cell::sync::Lazy;

pub static DARK_THEME: Lazy<ThemeSpec> = Lazy::new(|| ThemeSpec {
    color: "#cccccc",
    background_color: "#222222",
    link: "#66bbff",
    link_hover: "#40abcf",
    header_color: "#cccccc",
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
