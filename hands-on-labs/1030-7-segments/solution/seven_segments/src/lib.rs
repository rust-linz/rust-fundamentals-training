/// Generate characters to print for a 7-segment display.
/// 
/// The resulting vector contains tuples of the form (position, character).
/// The position is a byte where the higher 4 bits represent the x-coordinate
/// and the lower 4 bits represent the y-coordinate of the character.
/// X and y coordinates are between 0 and 2.
/// 
/// Here are all the digits with their representation:
/// 
/// ```txt
///      _   _       _   _   _   _   _   _
///   |  _|  _| |_| |_  |_    | |_| |_| | |
///   | |_   _|   |  _| |_|   | |_|  _| |_|
/// ```
/// 
/// # Arguments
/// 
/// * `byte` - The byte representing the 7-segment display.
///            This value is typically generated using the
///            `decimal_to_segments` function.
/// 
/// # Panics
/// 
/// This function panics if the input is not a valid 7-segment
/// representation (i.e. if it is larger than 0b111_1111).
pub fn draw_7_segments(byte: u8) -> Vec<(u8, char)> {
    if byte > 0b111_1111 {
        panic!("Invalid input: {}", byte);
    }

    let mut result = Vec::with_capacity(7);
    if byte & 0b000_0001 != 0 {
        result.push((0x10, '_'));
    }
    if byte & 0b000_0010 != 0 {
        result.push((0x21, '|'));
    }
    if byte & 0b000_0100 != 0 {
        result.push((0x22, '|'));
    }
    if byte & 0b000_1000 != 0 {
        result.push((0x12, '_'));
    }
    if byte & 0b001_0000 != 0 {
        result.push((0x02, '|'));
    }
    if byte & 0b010_0000 != 0 {
        result.push((0x01, '|'));
    }
    if byte & 0b100_0000 != 0 {
        result.push((0x11, '_'));
    }

    result
}

/// Convert a decimal digit to a 7-segment representation.
/// 
/// The resulting value is a byte where each bit represents
/// a segment of the 7-segment display. The bits are ordered
/// as follows (see also https://en.wikipedia.org/wiki/File:7_Segment_Display_with_Labeled_Segments.svg):
/// 
/// * bit 0 = segment A
/// * bit 1 = segment B
/// * ...
/// 
/// Therefore, the digit 0 is represented by the value 0b011_1111.
/// 
/// # Arguments
/// 
/// * `digit` - The decimal digit to convert.
/// 
/// # Panics
/// 
/// This function panics if the input is not a decimal digit (0-9).
pub fn decimal_to_segments(digit: u8) -> u8 {
    match digit {
        0 => 0b011_1111,
        1 => 0b000_0110,
        2 => 0b101_1011,
        3 => 0b100_1111,
        4 => 0b110_0110,
        5 => 0b110_1101,
        6 => 0b111_1101,
        7 => 0b000_0111,
        8 => 0b111_1111,
        9 => 0b110_1111,
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(0, 0b011_1111)]
    #[case(1, 0b000_0110)]
    #[case(2, 0b101_1011)]
    #[case(3, 0b100_1111)]
    #[case(4, 0b110_0110)]
    #[case(5, 0b110_1101)]
    #[case(6, 0b111_1101)]
    #[case(7, 0b000_0111)]
    #[case(8, 0b111_1111)]
    #[case(9, 0b110_1111)]
    fn test_decimal_to_segments(#[case] input: u8, #[case] expected: u8) {
        assert_eq!(decimal_to_segments(input), expected);
    }

    #[test]
    #[should_panic(expected = "unreachable")]
    fn test_decimal_to_segments_unreachable() {
        decimal_to_segments(10);
    }

    #[rstest]
    #[case(0b000_0001, vec![(0x10, '_')])]
    #[case(0b000_0010, vec![(0x21, '|')])]
    #[case(0b000_0100, vec![(0x22, '|')])]
    #[case(0b000_1000, vec![(0x12, '_')])]
    #[case(0b001_0000, vec![(0x02, '|')])]
    #[case(0b010_0000, vec![(0x01, '|')])]
    #[case(0b100_0000, vec![(0x11, '_')])]
    fn test_draw_7_segments(#[case] input: u8, #[case] expected: Vec<(u8, char)>) {
        assert_eq!(draw_7_segments(input), expected);
    }

    #[test]
    #[should_panic(expected = "Invalid input: 255")]
    fn test_draw_7_segments_invalid_input() {
        draw_7_segments(255);
    }
}
