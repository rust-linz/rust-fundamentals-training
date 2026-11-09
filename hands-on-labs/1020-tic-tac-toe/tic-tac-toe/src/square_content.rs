use std::fmt;

// Learning: Many traits support auto-implementation using the derive macro
// Todo: Copy this file into Rust Playground and choose Tools/Expand macros
//       to analyze what's going on behind the scenes
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SquareContent {
    X,
    O,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct InvalidSquareContent; // Note unit struct (field-less struct)

impl fmt::Display for InvalidSquareContent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Value does not represent a valid square content")
    }
}

impl std::error::Error for InvalidSquareContent {}

// Learning: There are system traits for type conversion. `From` is for conversions
//           that cannot fail, `TryFrom` is for conversions that can.
// Note: Take a look at unit tests to see how to do conversion.
impl TryFrom<u8> for SquareContent {
    type Error = InvalidSquareContent;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(SquareContent::X),
            2 => Ok(SquareContent::O),
            _ => Err(InvalidSquareContent),
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

impl TryFrom<char> for SquareContent {
    type Error = InvalidSquareContent;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'X' => Ok(SquareContent::X),
            'O' => Ok(SquareContent::O),
            _ => Err(InvalidSquareContent),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_into_u8() {
        // Learning: Implement From/TryFrom, get Into/TryInto for free.
        assert_eq!(Ok(SquareContent::X), SquareContent::try_from(1u8));
        assert_eq!(Ok(SquareContent::X), 1u8.try_into());
        assert_eq!(1, u8::from(SquareContent::X));
        assert_eq!(1u8, SquareContent::X.into());
    }

    #[test]
    fn from_into_char() {
        assert_eq!(Ok(SquareContent::X), SquareContent::try_from('X'));
        assert_eq!(Ok(SquareContent::X), 'X'.try_into());
        assert_eq!('X', char::from(SquareContent::X));
        assert_eq!('X', Into::<char>::into(SquareContent::X));
    }

    #[test]
    fn try_from_fails() {
        assert_eq!(Err(InvalidSquareContent), SquareContent::try_from(99u8));
        assert_eq!(Err(InvalidSquareContent), SquareContent::try_from('?'));
    }
}
