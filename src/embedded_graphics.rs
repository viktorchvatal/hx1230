use embedded_graphics_core::{prelude::*, pixelcolor::BinaryColor, Pixel};
use crate::{buffer::{ArrayDisplayBuffer, W, H}, DisplayBuffer};

impl DrawTarget for ArrayDisplayBuffer {
    type Color = BinaryColor;

    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where I: IntoIterator<Item = embedded_graphics_core::Pixel<Self::Color>> {
        for Pixel(coord, color) in pixels.into_iter() {
            if coord.x >= 0 && coord.x < W as i32 && coord.y >= 0 && coord.y/8 < H as i32 {
                let line = coord.y as usize / 8;
                let column = coord.x as usize;
                let shift = (coord.y as usize) % 8;

                if let Some(pixels) = self.get_line_mut(line) {
                    pixels[column] = pixels[column]
                        & (!(1 << shift))
                        | ((color.is_on() as u8) << shift);
                }
            }
        }

        Ok(())
    }
}

impl OriginDimensions for ArrayDisplayBuffer {
    fn size(&self) -> Size {
        Size::new(W as u32, (H*8) as u32)
    }
}