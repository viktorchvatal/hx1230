use crate::command::set_position;

/// Interface of HX1230 display driver allowing to transmit commands and data
///
/// There can be multiple implementations - either simple one implemented by
/// software using a bit banging, an implementation using a hardware
/// SPI interface or more advanced implementation using a DMA channel.
pub trait DisplayDriver {
    /// Send data to display
    fn send_data(&mut self, data: &[u8]) -> Result<(), ()>;

    /// Send multiple commands to display
    fn send_commands(&mut self, commands: &[u8]) -> Result<(), ()>;

    /// Send a single command to display
    fn send_command(&mut self, command: u8) -> Result<(), ()> {
        self.send_commands(&[command])
    }

    /// Send buffer of data
    fn send_buffer(&mut self, buffer: &dyn DisplayBuffer) -> Result<(), ()> {
        self.send_commands(&set_position(0, 0))?;

        for line_id in 0..buffer.line_count() {
            if let Some(ref line) = buffer.get_line(line_id) {
                self.send_data(line)?;
            }
        }

        Ok(())
    }
}

/// Trait representing a buffer of 1-bit display pixel data.
///
/// Each buffer line represents data of 8 pixel lines on the display,
/// each byte of the line represents data of a vertical 8-pixel column
pub trait DisplayBuffer {
    /// Display width in pixels
    fn width(&self) -> usize;

    /// Display height in pixel octets
    fn line_count(&self) -> usize;

    /// Line of pixel data, each byte represents one column of 8 pixel lines
    ///
    /// # Arguments
    /// * `y` - coordinate of a frame buffer line
    fn get_line(&self, y: usize) -> Option<&[u8]>;

    /// A mutable slice of one line of pixel data,
    /// each byte represents one column of 8 pixel lines
    ///
    /// # Arguments
    /// * `y` - coordinate of a frame buffer line
    fn get_line_mut(&mut self, y: usize) -> Option<&mut [u8]>;

    /// Set all bytes of display buffer with the specified value
    ///
    /// # Arguments
    /// * `value` - byte value to fill frame buffer with (0x00 clears all pixels,
    ///             0xff sets all pixels of the display)
    fn clear_buffer(&mut self, value: u8) {
        for y in 0..self.line_count() {
            self.clear_line(y, value)
        }
    }

    /// Set all bytes of the specified display buffer line `y` with the
    /// specified `value`
    ///
    /// # Arguments
    /// * `y` - coordinate of a frame buffer line
    /// * `value` - byte value to fill frame buffer with (0x00 clears all pixels,
    ///             0xff sets all pixels of the display)
    fn clear_line(&mut self, y: usize, value: u8) {
        if let Some(line) = self.get_line_mut(y) {
            line.iter_mut().for_each(|pixel| *pixel = value);
        }
    }
}