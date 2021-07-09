use std::ops::RangeInclusive;

use crate::{BoardIndex, Direction};

/*
    Learnings in this module:

    * Range-related traits
    * Custom iterators

    Recommended readings for this module:

    * `RangeInclusive` trait: https://doc.rust-lang.org/std/ops/struct.RangeInclusive.html
    * Iterators: https://doc.rust-lang.org/rust-by-example/trait/iter.html
*/

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct BoardIndexRangeInclusive(RangeInclusive<BoardIndex>);

impl BoardIndexRangeInclusive {
    pub fn new(start: BoardIndex, end: BoardIndex) -> BoardIndexRangeInclusive {
        if start.column() != end.column() && start.row() != end.row() {
            panic!("Invalid arguments, cannot construct inclusive board index range");
        }

        BoardIndexRangeInclusive(RangeInclusive::new(start, end))
    }

    pub fn length(&self) -> usize {
        if self.0.start().column() == self.0.end().column() {
           (self.0.start().row() as i8 - self.0.end().row() as i8).abs() as usize + 1
        } else {
           (self.0.start().column() as i8 - self.0.end().column() as i8).abs() as usize + 1
        }
    }
}

impl From<RangeInclusive<BoardIndex>> for BoardIndexRangeInclusive {
    fn from(r: RangeInclusive<BoardIndex>) -> Self {
        BoardIndexRangeInclusive::new(*r.start(), *r.end())
    }
}

pub struct BoardIndexIterator {
    current: Option<BoardIndex>,
    start: BoardIndex,
    end: BoardIndex,
    direction: Direction,
}

impl IntoIterator for BoardIndexRangeInclusive {
    type Item = BoardIndex;
    type IntoIter = BoardIndexIterator;

    fn into_iter(self) -> Self::IntoIter {
        BoardIndexIterator {
            current: None,
            start: *self.0.start(),
            end: *self.0.end(),
            direction: if self.0.start().column() == self.0.end().column() { Direction::Vertical } else { Direction::Horizontal },
        }
    }
}

impl Iterator for BoardIndexIterator {
    type Item = BoardIndex;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current {
            None => { self.current = Some(self.start); self.current },
            Some(c) => {
                match c.try_next(self.direction) {
                    Some(x) => if x <= self.end { self.current = Some(x); self.current } else { None },
                    None => None,
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ctor_valid_range_row() {
        BoardIndexRangeInclusive::new("A1".parse().unwrap(), "A3".parse().unwrap());
    }

    #[test]
    fn ctor_valid_range_col() {
        BoardIndexRangeInclusive::new("A1".parse().unwrap(), "C1".parse().unwrap());
    }

    #[test]
    #[should_panic]
    fn ctor_invalid_range() {
        BoardIndexRangeInclusive::new("A1".parse().unwrap(), "C3".parse().unwrap());
    }

    #[test]
    fn range_loop_row() {
        let mut ix = Vec::new();
        for i in BoardIndexRangeInclusive::from("A1".parse().unwrap()..="C1".parse().unwrap()) {
            ix.push(i);
        }

        assert_eq!(ix, vec!("A1".parse().unwrap(), "B1".parse().unwrap(), "C1".parse().unwrap()));
    }

    #[test]
    fn range_loop_col() {
        let mut ix = Vec::new();
        for i in BoardIndexRangeInclusive::from("A1".parse().unwrap()..="A3".parse().unwrap()) {
            ix.push(i);
        }

        assert_eq!(ix, vec!("A1".parse().unwrap(), "A2".parse().unwrap(), "A3".parse().unwrap()));
    }

    #[test]
    fn range_iter_row() {
        assert_eq!(3, BoardIndexRangeInclusive::from("A1".parse().unwrap()..="A3".parse().unwrap()).into_iter().count());
    }

    #[test]
    fn range_iter_col() {
        assert_eq!(3, BoardIndexRangeInclusive::from("A1".parse().unwrap()..="C1".parse().unwrap()).into_iter().count());
    }

    #[test]
    fn length_horizontal() {
        assert_eq!(3, BoardIndexRangeInclusive::from("A1".parse().unwrap()..="C1".parse().unwrap()).length());
    }

    #[test]
    fn length_vertical() {
        assert_eq!(3, BoardIndexRangeInclusive::from("A1".parse().unwrap()..="A3".parse().unwrap()).length());
    }

    #[test]
    fn length_single() {
        assert_eq!(1, BoardIndexRangeInclusive::from("A1".parse().unwrap()..="A1".parse().unwrap()).length());
    }
}
