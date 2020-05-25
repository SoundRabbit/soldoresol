use super::Color;

pub struct ColorSystem {}

#[allow(dead_code)]
impl ColorSystem {
    pub fn gray_100(alpha: u8) -> Color {
        Color::from([0xfa, 0xfb, 0xfc, alpha])
    }

    pub fn gray_200(alpha: u8) -> Color {
        Color::from([0xf6, 0xf8, 0xfa, alpha])
    }

    pub fn gray_900(alpha: u8) -> Color {
        Color::from([0x24, 0x29, 0x2e, alpha])
    }

    pub fn red_500(alpha: u8) -> Color {
        Color::from([0xd7, 0x3a, 0x49, alpha])
    }
}
