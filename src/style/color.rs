#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Color {
    pub fn with_alpha(&self, alpha: f32) -> AlphaColor {
        AlphaColor {
            red: self.red,
            green: self.green,
            blue: self.blue,
            alpha,
        }
    }

    pub fn to_css(&self) -> String {
        format!("rgb({}, {}, {})", self.red, self.green, self.blue)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct AlphaColor {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: f32,
}

impl AlphaColor {
    pub fn without_alpha(&self) -> Color {
        Color {
            red: self.red,
            green: self.green,
            blue: self.blue,
        }
    }

    pub fn to_css(&self) -> String {
        format!(
            "rgba({}, {}, {}, {})",
            self.red, self.green, self.blue, self.alpha
        )
    }
}
