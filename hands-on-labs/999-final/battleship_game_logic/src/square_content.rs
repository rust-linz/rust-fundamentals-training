/*
    Learnings in this module:

    * Create an enum
    * Generate trait implementations with `derive`
    * Learn about various important fundamental traits
    * Basic unit testing

    Recommended readings for this module:

    * `derive` macro: https://doc.rust-lang.org/reference/procedural-macros.html#derive-macros
    * `Copy` and `Clone` traits: https://doc.rust-lang.org/std/marker/trait.Copy.html
    * `PartialEq` and `Eq` traits: https://doc.rust-lang.org/std/cmp/trait.PartialEq.html
    * Enums: https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html
    * `Default` trait: https://doc.rust-lang.org/std/default/trait.Default.html
    * Standard prelude (reason why we do not need to import e.g. `Default`): https://doc.rust-lang.org/std/prelude/index.html
    * `match` keyword at https://doc.rust-lang.org/rust-by-example/flow_control/match.html
    * `From` and `Into` traits: https://doc.rust-lang.org/rust-by-example/conversion/from_into.html
    * `panic` macro: https://doc.rust-lang.org/std/macro.panic.html
    * Unit testing: https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html
*/

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum SquareContent {
    Water,
    Ship,
    HitShip,
    SunkenShip,
    Unknown,
}

impl SquareContent {
    pub fn is_ship(&self) -> bool {
        matches!(*self, SquareContent::Ship | SquareContent::HitShip | SquareContent::SunkenShip)
    }
}

impl Default for SquareContent {
    fn default() -> Self {
        SquareContent::Unknown
    }
}

impl From<u8> for SquareContent {
    fn from(value: u8) -> Self {
        match value {
            0 => SquareContent::Water,
            1 => SquareContent::Ship,
            2 => SquareContent::HitShip,
            3 => SquareContent::SunkenShip,
            4 => SquareContent::Unknown,
            v => panic!("Cannot convert {} to square content", v),
        }
    }
}

impl From<SquareContent> for u8 {
    fn from(c: SquareContent) -> Self {
        match c {
            SquareContent::Water => 0,
            SquareContent::Ship => 1,
            SquareContent::HitShip => 2,
            SquareContent::SunkenShip => 3,
            SquareContent::Unknown => 4,
        }
    }
}

// Discuss: Why `From` and not `Into`?
// See clippy warning docs at https://rust-lang.github.io/rust-clippy/master/index.html#from_over_into
impl From<SquareContent> for char {
    fn from(value: SquareContent) -> Self {
        match value {
            SquareContent::Water => '~',
            SquareContent::Ship => 'S',
            SquareContent::HitShip => 'h',
            SquareContent::SunkenShip => 'X',
            SquareContent::Unknown => ' ',
        }
    }
}

impl From<char> for SquareContent {
    fn from(value: char) -> Self {
        match value {
            '~' => SquareContent::Water,
            'S' => SquareContent::Ship,
            'h' => SquareContent::HitShip,
            'X' => SquareContent::SunkenShip,
            ' ' => SquareContent::Unknown,
            _ => panic!("Invalid character")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn content_default() {
        let c: SquareContent = Default::default();
        assert_eq!(SquareContent::Unknown, c);
    }

    #[test]
    fn from() {
        let c: SquareContent = SquareContent::from(1);
        assert_eq!(SquareContent::Ship, c);
    }

    #[test]
    #[should_panic(expected = "99")]
    fn from_fails() {
        SquareContent::from(99);
    }

    #[test]
    fn into_u8() {
        let c = SquareContent::Ship;
        let v: u8 = c.into();
        assert_eq!(1, v);
    }

    #[test]
    fn into_char() {
        let c = SquareContent::Ship;
        let v: char = c.into();
        assert_eq!('S', v);

        let v: char = char::from(SquareContent::Ship);
        assert_eq!('S', v);
    }

    #[test]
    fn from_char() {
        assert_eq!(SquareContent::Ship, 'S'.into());
    }
}
