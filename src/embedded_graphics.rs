use embedded_graphics_core::{prelude::*, pixelcolor::BinaryColor, Pixel};

impl<const W: usize, const H: usize> DrawTarget for ArrayDisplayBuffer<W, H> {
    type Color = BinaryColor;

    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where I: IntoIterator<Item = embedded_graphics_core::Pixel<Self::Color>> {
        for Pixel(coord, color) in pixels.into_iter() {
            if coord.x >= 0 && coord.x < W as i32 && coord.y >= 0 && coord.y/8 < H as i32 {
                let line = coord.y as usize / 8;
                let column = coord.x as usize;
                let shift = (coord.y as usize) % 8;

                self.pixels[line][column] = self.pixels[line][column]
                    & (!(1 << shift))
                    | ((color.is_on() as u8) << shift);
            }
        }

        Ok(())
    }
}

impl<const W: usize, const H: usize> OriginDimensions for ArrayDisplayBuffer<W, H> {
    fn size(&self) -> Size {
        Size::new(W as u32, (H*8) as u32)
    }
}