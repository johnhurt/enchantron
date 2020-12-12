const R_MASK: u32 = 0xFF000000;
const G_MASK: u32 = 0x00FF0000;
const B_MASK: u32 = 0x0000FF00;
const A_MASK: u32 = 0x000000FF;

const R_SHIFT: u8 = 32 - 8;
const G_SHIFT: u8 = R_SHIFT - 8;
const B_SHIFT: u8 = G_SHIFT - 8;

pub type Rgba32Color = u32;

pub trait Color {
    fn new(red: u8, green: u8, blue: u8, alpha: u8) -> Self;

    fn red(&self) -> u8;
    fn green(&self) -> u8;
    fn blue(&self) -> u8;
    fn alpha(&self) -> u8;
}

impl Color for Rgba32Color {
    fn new(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        ((red as u32) << R_SHIFT)
            + ((blue as u32) << B_SHIFT)
            + ((green as u32) << G_SHIFT)
            + (alpha as u32)
    }

    fn red(&self) -> u8 {
        ((*self & R_MASK) >> R_SHIFT) as u8
    }

    fn green(&self) -> u8 {
        ((*self & G_MASK) >> G_SHIFT) as u8
    }

    fn blue(&self) -> u8 {
        ((*self & B_MASK) >> B_SHIFT) as u8
    }

    fn alpha(&self) -> u8 {
        (*self & A_MASK) as u8
    }
}
