use crate::command::set_position;

/// Interface of HX1230 display driver allowing to transmit commands and data
///
/// There can be multiple implementations - either simple one implemented by
/// software using a bit banging, an implementation using a hardware
/// SPI interface or more advanced implementation using a DMA channel.
pub trait DisplayDriver {
    /// Send data to display
    fn data(&mut self, data: &[u8]) -> Result<(), ()>;

    /// Send multiple commands to display
    fn commands(&mut self, commands: &[u8]) -> Result<(), ()>;

    /// Send a single command to display
    fn command(&mut self, command: u8) -> Result<(), ()> {
        self.commands(&[command])
    }

    /// Send buffer of data
    fn buffer(&mut self, buffer: &dyn DisplayBuffer) -> Result<(), ()> {
        self.commands(&set_position(0, 0))?;

        for line_id in 0..buffer.line_count() {
            if let Some(ref line) = buffer.get_line(line_id) {
                self.data(line)?;
            }
        }

        Ok(())
    }
}

/// Trait representing a buffer of display pixel data
pub trait DisplayBuffer {
    /// Display width in pixels
    fn width(&self) -> usize;

    /// Display height in pixel octets
    fn line_count(&self) -> usize;

    /// Line of pixel data, each byte represents one column of 8 pixel lines
    fn get_line(&self, y: usize) -> Option<&[u8]>;

    /// A mutable slice of one line of pixel data,
    /// each byte represents one column of 8 pixel lines
    fn get_line_mut(&mut self, y: usize) -> Option<&mut [u8]>;
}