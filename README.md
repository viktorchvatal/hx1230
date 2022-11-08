# HX1230 Display driver

Early version of HX1230 display driver running on `embedded-hal`

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

Example code: [examples/graphics.rs](examples/graphics.rs)

Note:
 - openocd must be running to successfully run the example
 - MCU memory layout must match the one specified in the `memory.x` file
 - GDB must successfully apply `.gdbinit` file present in the root crate directory

To run unit tests on the local machine (change the target in case of different platform)

```
test --lib --target x86_64-unknown-linux-gnu
```
