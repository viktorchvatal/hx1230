
/// Display software reset
pub const fn reset() -> u8 {
    0xE2
}

/// Turn on power
pub const fn power_on() -> u8 {
    0x2F
}

/// Turn off power
pub const fn power_off() -> u8 {
    0x28
}

/// Turn on horizontal mirror
pub const fn horizontal_flip_on() -> u8 {
    0xA1
}

/// Turn off horizontal mirror
pub const fn horizontal_flip_off() -> u8 {
    0xA0
}

/// Turn on vertical mirror
pub const fn vertical_flip_on() -> u8 {
    0xC8
}

/// Turn off vertical mirror
pub const fn vertical_flip_off() -> u8 {
    0xC0
}

/// Enable display test (all pixels active)
pub const fn display_test_on() -> u8 {
    0xA5
}

/// Disable display test (all pixels active)
pub const fn display_test_off() -> u8 {
    0xA4
}

/// Enable inverted pixels (negative image)
pub const fn invert_on() -> u8 {
    0xA7
}

/// Disable inverted pixels (negative image)
pub const fn invert_off() -> u8 {
    0xA6
}

/// Turn on display
pub const fn display_on() -> u8 {
    0xAF
}

/// Turn off display
pub const fn display_off() -> u8 {
    0xAE
}

/// Set display contrast (0 - 31)
pub const fn set_contrast(contrast: u8) -> u8 {
    0x80 | (0b00011111 & contrast)
}

/// Set display line (0 - 63)
pub const fn set_line(line: u8) -> u8 {
    0x40 | (0b00111111 & line)
}

/// Set page (0 - 9) - y coordinate byte
pub const fn set_page(page: u8) -> u8 {
    0xB0 | (0b00001111 & page)
}

/// Set column low 4 bits (0 - 63) - x coordinate
pub const fn set_column_low(column: u8) -> u8 {
    0b00001111 & column
}

/// Set column high 3 bits (0 - 63) - x coordinate
pub const fn set_column_high(column: u8) -> u8 {
    0b00010000 | ((column >> 4) & 0b00000111)
}

/// Sequence of commands to fully initialize the display
pub const fn init_sequence() -> &'static [u8] {
    const INIT: &[u8] = &[
        power_on(),
        set_contrast(30),
        display_test_off(),
        horizontal_flip_off(),
        vertical_flip_off(),
        invert_off(),
        display_on(),
        set_column_low(0),
        set_column_high(0),
        set_page(0),
    ];

    INIT
}

/// Sequence of commands to fully select position (row and column)
pub const fn set_position(column: u8, row: u8) -> [u8; 3] {
    [
        set_column_low(column),
        set_column_high(column),
        set_page(row),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_position() {
        let commands = set_position(13, 7);
        assert_eq!(commands, [0b00001101, 0b00010000, 0b10110111]);
    }
}