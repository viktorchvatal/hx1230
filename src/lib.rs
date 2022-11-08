//! Simple HX1230 display driver to be used in embedded environment

#![no_std]
#![deny(unsafe_code)]

pub mod command;
mod spi_driver;
mod interface;
mod buffer;
#[cfg(feature = "embedded_graphics")]
mod embedded_graphics;

pub use interface::{DisplayDriver, DisplayBuffer};
pub use spi_driver::SpiDriver;
pub use buffer::ArrayDisplayBuffer;
