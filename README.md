# HX1230 Display driver

Early version of HX1230 display driver running on `embedded-hal` optionally
integrated with `embedded_graphics` library

[![HX1320 display module](doc/display.jpg?raw=true)](examples/graphics.rs)

## State of the library

The library is at an early state of development, but usable

### What's working

 - communication with the HX1230 display using a SPI interface
 - integration with the `embedded_graphics` library

### Caveats

 - `embedded_graphics` integration is only basic with no further optimizations for faster rendering
 - there is no driver variant using DMA channel for data transmission
 - unit tests are sparse (yet)

## Examples

Library has been tested with STM32F103C8T6 microcontroller

To run example on such MCU, run

```
cargo run --example graphics --release
```

### Usage

Initialize the display

```rust
// Create display driver using the provided SPI interface and chip select pin
let mut display = SpiDriver::new(&mut spi, &mut display_cs);

// Send the initialization sequence
display.initialize(&mut delay).unwrap();

// Create frame buffer for HX1230 display
let mut frame_buffer: ArrayDisplayBuffer = ArrayDisplayBuffer::new();
```

Do some drawing using `embedded_graphics` into buffer

```rust
let text_style = MonoTextStyle::new(&FONT_6X13, BinaryColor::On);

Text::new("example", Point::new(0, 12), text_style)
    .draw(&mut frame_buffer)
    .unwrap();
```

Send data to display

```rust
// Send display buffer data to display
display.send_buffer(&frame_buffer).unwrap();
```

Full example code: [examples/graphics.rs](examples/graphics.rs)

Note:
 - openocd must be running to successfully run the example
 - MCU memory layout must match the one specified in the `memory.x` file
 - GDB must successfully apply `.gdbinit` file present in the root crate directory

To run unit tests on the local machine (change the target in case of different platform)

```
test --lib --target x86_64-unknown-linux-gnu
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Resources

Python implementation of HX1230 display driver, including useful wiring information
and even product datasheets: https://github.com/mcauser/micropython-hx1230