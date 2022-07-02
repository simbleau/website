use crate::themes::Color;

#[derive(Debug, Clone)]
pub struct Theme {
    // Foreground color 1
    pub fg1: Color,
    // Foreground color 2
    pub fg2: Color,
    // Background color 1
    pub bg1: Color,
    // Background color 2
    pub bg2: Color,
    // Accent color 1
    pub ac1: Color,
    // Accent color 2
    pub ac2: Color,
}
