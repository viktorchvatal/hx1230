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
}

