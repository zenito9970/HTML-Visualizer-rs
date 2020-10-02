use std::fmt::{self, Display};

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    pub const BLACK: Color = Color { r: 0, g: 0, b: 0 };
    pub const WHITE: Color = Color { r: 255, g: 255, b: 255 };
    pub const GRAY: Color = Color { r: 150, g: 150, b: 150 };
    pub const RED: Color = Color { r: 223, g: 71, b: 87 };
    pub const GREEN: Color = Color { r: 100, g: 216, b: 127 };
    pub const BLUE: Color = Color { r: 83, g: 146, b: 210 };

    pub fn new<T: Into<u8>>(r: T, g: T, b: T) -> Color {
        let (r, g, b) = (r.into(), g.into(), b.into());
        Color { r, g, b }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "rgb({},{},{})", self.r, self.g, self.b)
    }
}
