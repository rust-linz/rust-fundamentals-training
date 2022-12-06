// Learning: Many traits support auto-implementation using the derive macro
// Todo: Copy this file into Rust Playground and choose Tools/Expand macros 
//       to analyze what's going on behind the scenes
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SquareContent {
    X,
    O,
}

// Learning: There are system traits for type conversion.
// Note: Take a look at unit tests to see how to do conversion.
impl From<u8> for SquareContent {
    fn from(value: u8) -> Self {
        match value {
            1 => SquareContent::X,
            2 => SquareContent::O,
            v => panic!("Cannot convert {v} to square content"),
        }
    }
}

impl From<SquareContent> for u8 {
    fn from(c: SquareContent) -> Self {
        match c {
            SquareContent::X => 1,
            SquareContent::O => 2,
        }
    }
}

impl From<SquareContent> for char {
    fn from(value: SquareContent) -> Self {
        match value {
            SquareContent::X => 'X',
            SquareContent::O => 'O',
        }
    }
}

impl From<char> for SquareContent {
    fn from(value: char) -> Self {
        match value {
            'X' => SquareContent::X,
            'O' => SquareContent::O,
            v => panic!("Invalid character {v}")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_into_u8() {
        // Learning: Implement From, get Into for free.
        assert_eq!(SquareContent::X, SquareContent::from(1));
        assert_eq!(SquareContent::X, 1.into());
        assert_eq!(1, u8::from(SquareContent::X));
        assert_eq!(1u8, SquareContent::X.into());
    }

    #[test]
    fn from_into_char() {
        assert_eq!(SquareContent::X, SquareContent::from('X'));
        assert_eq!(SquareContent::X, 'X'.into());
        assert_eq!('X', char::from(SquareContent::X));
        assert_eq!('X', Into::<char>::into(SquareContent::X));
    }

    #[test]
    #[should_panic(expected = "99")]
    #[allow(unused_must_use)]
    fn from_fails() {
        SquareContent::from(99);
    }
}
