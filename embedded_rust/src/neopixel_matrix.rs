use embedded_hal::digital::v2::OutputPin;
use smart_leds::{RGB8, SmartLedsWrite};

pub const MATRIX_WIDTH: usize = 8;
pub const MATRIX_HEIGHT: usize = 8;
pub const MATRIX_PIXELS: usize = MATRIX_WIDTH * MATRIX_HEIGHT;

pub fn urgb_u32(r: u8, g: u8, b: u8) -> u32 {
    ((r as u32) << 8) | ((g as u32) << 16) | (b as u32)
}

fn u32_to_rgb8(color: u32) -> RGB8 {
    let r = ((color >> 8) & 0xFF) as u8;
    let g = ((color >> 16) & 0xFF) as u8;
    let b = (color & 0xFF) as u8;
    RGB8 { r, g, b }
}

pub struct NeoMatrix<PWR>
where
    PWR: OutputPin,
{
    width: usize,
    height: usize,
    power_pin: PWR,
    pixels: [u32; MATRIX_PIXELS],
}

impl<PWR> NeoMatrix<PWR>
where
    PWR: OutputPin,
{
    pub fn new(power_pin: PWR) -> Self {
        Self {
            width: MATRIX_WIDTH,
            height: MATRIX_HEIGHT,
            power_pin,
            pixels: [0; MATRIX_PIXELS],
        }
    }

    pub fn init(&mut self) {
        let _ = self.power_pin.set_high();
        self.clear_pixels();
    }

    pub fn set_pixel(&mut self, row: usize, col: usize, color: u32) {
        if row >= self.height || col >= self.width {
            return;
        }

        let index = self.xy_to_index(row, col);
        self.pixels[index] = color;
    }

    pub fn clear_pixels(&mut self) {
        for pixel in self.pixels.iter_mut() {
            *pixel = 0;
        }
    }

    pub fn load_frame(&mut self, frame: &[u32; MATRIX_PIXELS]) {
        self.pixels.copy_from_slice(frame);
    }

    pub fn write<W, E>(&mut self, writer: &mut W) -> Result<(), E>
    where
        W: SmartLedsWrite<Color = RGB8, Error = E>,
    {
        let data = self.pixels.iter().copied().map(u32_to_rgb8);
        writer.write(data)
    }

    fn xy_to_index(&self, row: usize, col: usize) -> usize {
        row * self.width + col
    }
}