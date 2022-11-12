#![no_std]
#![no_main]

use core::fmt::Write;
use arrayvec::ArrayString;
use embedded_graphics::{prelude::*, Drawable, pixelcolor::BinaryColor, text::Text};
use embedded_graphics::primitives::{PrimitiveStyle, Circle};
use embedded_graphics::mono_font::{ascii::FONT_6X13, MonoTextStyle};
use embedded_hal::{spi::{Mode, Phase, Polarity}};
use cortex_m_rt::entry;
use hx1230::{ArrayDisplayBuffer, DisplayBuffer, SpiDriver, DisplayDriver};
use stm32f1xx_hal::{pac, prelude::*, spi::{NoMiso, Spi}};
use panic_halt as _;

pub const SPI_MODE: Mode = Mode {
    phase: Phase::CaptureOnFirstTransition,
    polarity: Polarity::IdleLow,
};

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();

    // Freeze the configuration of all the clocks in the system and store
    // the frozen frequencies in `clocks`
    let clocks = rcc.cfgr.use_hse(8.MHz()).sysclk(72.MHz()).hclk(8.MHz()).freeze(&mut flash.acr);

    let mut gpiob = dp.GPIOB.split();

    // Display CS (chip select) control pin
    let mut display_cs = gpiob.pb12.into_push_pull_output(&mut gpiob.crh);

    // SPI2, we use only output, so there is no miso input
    let sck = gpiob.pb13.into_alternate_push_pull(&mut gpiob.crh);
    let mosi = gpiob.pb15.into_alternate_push_pull(&mut gpiob.crh);

    // Create SPI interface
    let mut spi = Spi::spi2(dp.SPI2, (sck, NoMiso, mosi), SPI_MODE, 4.MHz(), clocks,);

    let mut delay = cp.SYST.delay(&clocks);

    // Create display driver using the provided SPI interface and chip select pin
    let mut display = SpiDriver::new(&mut spi, &mut display_cs);

    // Send the initialization sequence
    display.initialize(&mut delay).unwrap();

    // Create frame buffer for HX1230 display
    let mut frame_buffer: ArrayDisplayBuffer = ArrayDisplayBuffer::new();

    let mut diameter = 1;
    let text_style = MonoTextStyle::new(&FONT_6X13, BinaryColor::On);

    loop {
        // Fill the frame buffer with zeros
        frame_buffer.clear_buffer(0x00);

        // Draw some circles
        draw_circle(48, 40, (diameter + 10) % 80, &mut frame_buffer);
        draw_circle(20, 20, (diameter +  0) % 60, &mut frame_buffer);
        draw_circle(60, 20, (diameter + 20) % 60, &mut frame_buffer);
        draw_circle(80, 50, (diameter + 30) % 60, &mut frame_buffer);
        draw_circle(20, 60, (diameter + 40) % 60, &mut frame_buffer);

        // Clear the top 96x16 pixels with zeros
        frame_buffer.clear_line(0, 0x00);
        frame_buffer.clear_line(1, 0x00);

        // Render some text
        let mut text = ArrayString::<14>::new();
        let _ = write!(&mut text, "Hello Rust {}", diameter);

        Text::new(&text, Point::new(0, 12), text_style)
            .draw(&mut frame_buffer)
            .unwrap();

        // Send display buffer data to display
        display.send_buffer(&frame_buffer).unwrap();

        diameter = diameter + 1;

        // Wait for next frame
        delay.delay_ms(100_u16);
    }
}

/// Draw a circle using the embedded_graphics library
fn draw_circle<D>(x: i32, y: i32, diameter: i32, frame_buffer: &mut D)
where D: DrawTarget<Color = BinaryColor> {
    let _result = Circle::new(Point::new(x - diameter/2, y - diameter/2), diameter as u32)
        .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 3))
        .draw(frame_buffer);
}
