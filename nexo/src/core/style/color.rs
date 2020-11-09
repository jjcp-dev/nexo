#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Color {
    value: u32,
}

impl Color {
    #[inline]
    pub fn rgb(r: u8, g: u8, b: u8) -> Color {
        Color {
            value: (r as u32) << 24 | (g as u32) << 16 | (b as u32) << 8 | 0xFF,
        }
    }

    #[inline]
    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color {
            value: (r as u32) << 24 | (g as u32) << 16 | (b as u32) << 8 | a as u32,
        }
    }

    #[inline]
    pub fn white() -> Color {
        Color::rgb(255, 255, 255)
    }

    #[inline]
    pub fn black() -> Color {
        Color::rgb(0, 0, 0)
    }

    #[inline]
    pub fn r(&self) -> u8 {
        (self.value >> 24) as u8
    }

    #[inline]
    pub fn g(&self) -> u8 {
        (self.value >> 16) as u8
    }

    #[inline]
    pub fn b(&self) -> u8 {
        (self.value >> 8) as u8
    }

    #[inline]
    pub fn a(&self) -> u8 {
        self.value as u8
    }

    #[inline]
    pub fn value(&self) -> u32 {
        self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn white_returns_a_white_opaque_color() {
        let c = Color::white();

        assert_eq!(c.r(), 255);
        assert_eq!(c.g(), 255);
        assert_eq!(c.b(), 255);
        assert_eq!(c.a(), 255);
    }

    #[test]
    fn black_returns_a_black_opaque_color() {
        let c = Color::black();

        assert_eq!(c.r(), 0);
        assert_eq!(c.g(), 0);
        assert_eq!(c.b(), 0);
        assert_eq!(c.a(), 255);
    }

    #[test]
    fn rgb_given_primary_colors_returns_an_opaque_color() {
        let c = Color::rgb(255, 200, 150);

        assert_eq!(c.r(), 255);
        assert_eq!(c.g(), 200);
        assert_eq!(c.b(), 150);
        assert_eq!(c.a(), 255);
    }

    #[test]
    fn rgba_given_primary_colors_and_alpha_value_returns_a_color() {
        let c = Color::rgba(255, 200, 150, 100);

        assert_eq!(c.r(), 255);
        assert_eq!(c.g(), 200);
        assert_eq!(c.b(), 150);
        assert_eq!(c.a(), 100);
    }

    #[test]
    fn r_given_a_color_returns_the_red_component() {
        let c = Color::rgba(50, 100, 150, 200);

        assert_eq!(c.r(), 50);
    }

    #[test]
    fn g_given_a_color_returns_the_green_component() {
        let c = Color::rgba(50, 100, 150, 200);

        assert_eq!(c.g(), 100);
    }

    #[test]
    fn b_given_a_color_returns_the_blue_component() {
        let c = Color::rgba(50, 100, 150, 200);

        assert_eq!(c.b(), 150);
    }

    #[test]
    fn a_given_a_color_returns_the_alpha_component() {
        let c = Color::rgba(50, 100, 150, 200);

        assert_eq!(c.a(), 200);
    }

    #[test]
    fn value_given_a_color_returns_the_internal_32_bit_representation() {
        let c = Color::rgba(0xFF, 0xAA, 0x49, 0x22);

        assert_eq!(c.value(), 0xFFAA4922);
    }
}
