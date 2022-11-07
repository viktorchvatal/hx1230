#![no_std]
#![deny(unsafe_code)]

pub mod command;
mod spi_driver;
mod interface;

pub use interface::DisplayDriver;
pub use spi_driver::SpiDriver;
