use super::Color;

impl Color {
    pub fn to_rgb(&self) -> (u8, u8, u8) {
        match self {
            Self::White => (255, 255, 255),
            Self::Black => (0, 0, 0),
            Self::LightGray => (169, 169, 169),
            Self::DarkGray => (211, 211, 211),
        }
    }
}
