use super::Color;

pub trait HasMutableColor {
    type C: Color;

    fn set_color(&self, color: Self::C);

    fn create_8_bit_color(r: u8, g: u8, b: u8, a: u8) -> Self::C {
        <Self::C as Color>::new(r, g, b, a)
    }

    fn set_8_bit_color(&self, r: u8, g: u8, b: u8, a: u8) {
        self.set_color(Self::create_8_bit_color(r, g, b, a))
    }
}
