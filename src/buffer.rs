use crate::interface::DisplayBuffer;

/// Display width in pixels
pub const W: usize = 96;

/// Display width in octets
pub const H: usize = 9;

/// Fixed size display frame buffer for HX1230 display,
pub struct ArrayDisplayBuffer {
    pixels: [[u8; W]; H]
}

impl ArrayDisplayBuffer {
    /// Create a new buffer for the HX1230 display
    pub fn new() -> Self {
        Self {
            pixels: [[0; W]; H],
        }
    }
}

impl DisplayBuffer for ArrayDisplayBuffer {
    fn get_line(&self, y: usize) -> Option<&[u8]> {
        self.pixels.get(y).map(|array| array.as_slice())
    }

    fn get_line_mut(&mut self, y: usize) -> Option<&mut [u8]> {
        self.pixels.get_mut(y).map(|array| array.as_mut_slice())
    }

    fn width(&self) -> usize {
        W
    }

    fn line_count(&self) -> usize {
        H
    }
}