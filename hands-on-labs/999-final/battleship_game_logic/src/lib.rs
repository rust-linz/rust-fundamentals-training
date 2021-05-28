pub mod board_content {
    use std::{fmt, ops::{Index, IndexMut}};

    #[derive(Debug, Copy, Clone, PartialEq)]
    // Read more about Copy and Clone traits at https://doc.rust-lang.org/std/marker/trait.Copy.html
    // Read more about derive macro at https://doc.rust-lang.org/reference/procedural-macros.html#derive-macros
    pub enum SquareContent {
        // Read more about enums at https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html
        Unknown,
        Water,
        Ship,
        HitShip,
        SunkenShip,
    }

    const BOARD_SIDE_LENGTH: usize = 10; // Read more about constants at https://doc.rust-lang.org/std/keyword.const.html
    const BOARD_SIZE: usize = BOARD_SIDE_LENGTH * BOARD_SIDE_LENGTH;

    #[derive(Debug)]
    pub struct BoardContent {
        // Read more about structs at https://doc.rust-lang.org/book/ch05-00-structs.html
        board_content: [SquareContent; BOARD_SIZE],
        // Read more about arrays at https://doc.rust-lang.org/book/ch03-02-data-types.html#the-array-type
    }

    #[derive(Debug, Copy, Clone, PartialEq)]
    pub enum ConversionError {
        // Read more about error types at https://doc.rust-lang.org/rust-by-example/error/multiple_error_types/define_error_type.html
        InvalidBoardSize,
        InvalidSquareValue { index: usize, value: u8 },
    }

    impl fmt::Display for ConversionError {
        // Read more about formatters at https://doc.rust-lang.org/std/fmt/index.html
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                ConversionError::InvalidBoardSize => write!(
                    f,
                    "Board data is of invalid size. Has to contain {size} elements",
                    size = BOARD_SIZE
                ),
                ConversionError::InvalidSquareValue { index, value } => write!(
                    f,
                    "Board data contains invalid value {v} at index {ix}",
                    v = value,
                    ix = index
                ),
            }
        }
    }

    impl BoardContent {
        // Read more about the imple keyword at https://doc.rust-lang.org/std/keyword.impl.html
        pub fn new() -> BoardContent {
            // Read more about naming conventions at https://rust-lang.github.io/api-guidelines/naming.html
            BoardContent::from_content(SquareContent::Unknown)
        }

        pub fn from_content(initial_content: SquareContent) -> BoardContent {
            BoardContent {
                board_content: [initial_content; BOARD_SIZE],
            }
        }

        pub fn from_bytes(bytes: &[u8]) -> Result<BoardContent, ConversionError> {
            match bytes.len() {
                // Read more about the match keyword at https://doc.rust-lang.org/rust-by-example/flow_control/match.html
                BOARD_SIZE => {
                    let mut content = BoardContent::new();
                    for (ix, square_byte) in bytes.iter().enumerate() {
                        content.board_content[ix] = match square_byte {
                            0 => SquareContent::Unknown,
                            1 => SquareContent::Water,
                            2 => SquareContent::Ship,
                            3 => SquareContent::HitShip,
                            4 => SquareContent::SunkenShip,
                            v => {
                                return Err(ConversionError::InvalidSquareValue {
                                    index: ix,
                                    value: v.to_owned(),
                                })
                            }
                        }
                    }

                    Ok(content)
                }
                _ => Err(ConversionError::InvalidBoardSize),
            }
        }

        pub fn iter(&self) -> impl Iterator<Item = &SquareContent> {
            // Read more about impl Trait at https://doc.rust-lang.org/rust-by-example/trait/impl_trait.html
            self.board_content.iter()
        }
    }

    impl Index<usize> for BoardContent {
        // Read more about the index trait at https://doc.rust-lang.org/std/ops/trait.Index.html
        type Output = SquareContent;

        fn index(&self, ix: usize) -> &Self::Output {
            if ix >= BOARD_SIZE {
                panic!("Index out of bounds");
                // Read more about panicing at https://doc.rust-lang.org/rust-by-example/error/panic.html
            }

            &self.board_content[ix]
        }
    }

    impl IndexMut<usize> for BoardContent {
        // Read more about the index mut trait at https://doc.rust-lang.org/std/ops/trait.IndexMut.html
        fn index_mut(&mut self, ix: usize) -> &mut Self::Output {
            if ix >= BOARD_SIZE {
                panic!("Index out of bounds");
            }

            &mut self.board_content[ix]
        }
    }
}

#[cfg(test)] // Read more about unit testing at https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html
mod tests {
    use super::board_content::*;

    #[test]
    fn create_new_board_content() {
        let b = BoardContent::new();
        assert_eq!(b.iter().count(), 100);
        assert!(b.iter().all(|v| *v == SquareContent::Unknown));
    }

    #[test]
    fn create_initialized_board() {
        let b = BoardContent::from_content(SquareContent::Water);
        assert_eq!(b.iter().count(), 100);
        assert!(b.iter().all(|v| *v == SquareContent::Water));
    }

    #[test]
    fn from_bytes_wrong_length() {
        let b = BoardContent::from_bytes(&[1, 2, 3]);
        assert_eq!(b.err(), Some(ConversionError::InvalidBoardSize));
    }

    #[test]
    fn from_bytes_wrong_value() {
        let mut bytes = [0; 100];
        bytes[99] = 99;

        let b = BoardContent::from_bytes(&bytes);
        assert_eq!(
            b.err(),
            Some(ConversionError::InvalidSquareValue {
                index: 99,
                value: 99
            })
        );
    }

    #[test]
    fn from_bytes() {
        let b = BoardContent::from_bytes(&[0; 100]).unwrap();
        assert_eq!(b.iter().count(), 100);
        assert!(b.iter().all(|v| *v == SquareContent::Unknown));
    }

    #[test]
    #[should_panic]
    fn indexing_out_of_bounds() {
        let b = BoardContent::new();
        b[100];
    }

    #[test]
    #[should_panic]
    fn indexing_mut_out_of_bounds() {
        let mut b = BoardContent::new();
        b[100] = SquareContent::SunkenShip;
    }

    #[test]
    fn indexing() {
        let mut b = BoardContent::new();
        b[99] = SquareContent::SunkenShip;
        assert_eq!(SquareContent::SunkenShip, b[99]);
    }
}
