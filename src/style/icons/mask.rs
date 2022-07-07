use std::fmt;

#[derive(PartialEq, Clone, Copy)]
pub enum IconMask {
    HalfFill,
}

impl IconMask {
    pub fn data(&self) -> &'static str {
        match self {
            HalfFill => {
                r#"data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24'%3E%3Cpath d='M12 0C6 0 0 6 0 12s5 12 12 12 12-5 12-12S19 0 12 0Zm0 4c5 0 8 3 8 8s-3 8-8 8V4Z'/%3E%3C/svg%3E"#
            }
        }
    }
}

impl fmt::Display for IconMask {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.data())
    }
}
