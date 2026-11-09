#![allow(dead_code)] // Note: To avoid warnings for unused code in this demo file

use std::{fmt, ops::Index};

use crate::{
    board_content::{BOARD_SIDE_LENGTH, BoardContent},
    square_content::SquareContent,
};

// Represents a single row in BoardContent
pub struct Row<'a> {
    // Note: Lifetime annotation here means that the struct `Row`
    // cannot outlive the reference it holds in the `board` field.
    board: &'a BoardContent,
    pub row_index: usize,
}

impl<'a> Row<'a> {
    pub fn new(board: &'a BoardContent, row: usize) -> Self {
        Row {
            board,
            row_index: row,
        }
    }

    pub fn as_slice(&self) -> &[Option<SquareContent>] {
        &self.board.as_slice()
            [(self.row_index * BOARD_SIDE_LENGTH)..((self.row_index + 1) * BOARD_SIDE_LENGTH)]
    }

    pub fn has_next(&self) -> bool {
        self.row_index < BOARD_SIDE_LENGTH - 1
    }
}

// Nothing new here, just another indexer (read-only)
impl<'a> Index<usize> for Row<'a> {
    type Output = Option<SquareContent>;

    fn index(&self, ix: usize) -> &Self::Output {
        if ix >= BOARD_SIDE_LENGTH {
            panic!("Index out of bounds");
        }

        &self.board[self.row_index * BOARD_SIDE_LENGTH + ix]
    }
}

pub struct RowsIterator<'a> {
    board: &'a BoardContent,
    next_row: usize,
}

impl<'a> RowsIterator<'a> {
    pub fn new(board: &'a BoardContent) -> Self {
        RowsIterator { board, next_row: 0 }
    }
}

// Learning: How to implement a custom iterator
impl<'a> Iterator for RowsIterator<'a> {
    type Item = Row<'a>;

    fn next(&mut self) -> Option<Row<'a>> {
        if self.next_row >= BOARD_SIDE_LENGTH {
            return None;
        }

        let current = self.next_row;
        self.next_row += 1;
        Some(Row {
            board: self.board,
            row_index: current,
        })
    }
}

// Learning: Implement a trait with a lifetime annotation.
// Note: Lifetime annotation here means that the iterator
//       cannot outlive the struct implementing the iterator.
pub trait Rows {
    fn row(&self, row: usize) -> Row<'_>;

    // Learn: Dynamic binding instead of static binding
    fn rows(&self) -> Box<dyn Iterator<Item = Row<'_>> + '_>;
}

// Learning: Implement our trait for BoardContent
impl Rows for BoardContent {
    fn row(&self, row: usize) -> Row<'_> {
        if row >= BOARD_SIDE_LENGTH {
            panic!("Index out of bounds");
        }

        Row::new(self, row)
    }

    fn rows(&self) -> Box<dyn Iterator<Item = Row<'_>> + '_> {
        Box::new(RowsIterator::new(self))
    }
}

impl fmt::Display for BoardContent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fn build_separator(chars: &[char]) -> String {
            debug_assert!(chars.len() == 4);

            let mut result = String::new();
            result.reserve_exact(
                chars[0].len_utf8()
                    + BOARD_SIDE_LENGTH * 2 * chars[1].len_utf8()
                    + (BOARD_SIDE_LENGTH - 1) * chars[2].len_utf8()
                    + chars[3].len_utf8()
                    + '\n'.len_utf8(),
            );
            result.push(chars[0]);
            for _ in 0..BOARD_SIDE_LENGTH - 1 {
                result.push(chars[1]);
                result.push(chars[1]);
                result.push(chars[2]);
            }

            result.push(chars[1]);
            result.push(chars[1]);
            result.push(chars[3]);

            result.push('\n');

            result
        }

        let top = build_separator(&['┏', '━', '┯', '┓']);
        let middle = build_separator(&['┠', '─', '┼', '┨']);
        let bottom = build_separator(&['┗', '━', '┷', '┛']);

        let mut result = String::new();
        result.reserve_exact(
            top.len()
                + middle.len() * (BOARD_SIDE_LENGTH - 1)
                + bottom.len()
                + (2 * '┃'.len_utf8()
                    + (BOARD_SIDE_LENGTH - 1) * '|'.len_utf8()
                    + BOARD_SIDE_LENGTH * 2 * ' '.len_utf8()
                    + '\n'.len_utf8())
                    * BOARD_SIDE_LENGTH,
        );

        result.push_str(&top);

        // Note: Here we use our rows iterator
        for row in self.rows() {
            result.push('┃');
            for col in 0..BOARD_SIDE_LENGTH {
                let char = match row[col] {
                    None => ' ',
                    Some(x) => x.into(),
                };
                result.push(char);
                result.push(char);
                if col < BOARD_SIDE_LENGTH - 1 {
                    result.push('|');
                }
            }

            result.push('┃');
            result.push('\n');
            if row.row_index < BOARD_SIDE_LENGTH - 1 {
                result.push_str(&middle);
            }
        }

        result.push_str(&bottom);

        write!(f, "{result}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slice_row() {
        let mut b = BoardContent::new();
        let square_content = Some(SquareContent::X);
        b[3] = square_content;

        // Note that we need the following variable to extend the lifetime of
        // the `Row` instance. Otherwise, the slice returned by `as_slice()` would
        // outlive our temporary row. So the following line would NOT work:
        // let rs = b.row(1).as_slice(); // <<< DOES NOT COMPILE
        let row = b.row(1);
        let rs = row.as_slice();
        assert_eq!(rs.len(), 3);
        assert_eq!(rs[0], square_content);
    }

    #[test]
    fn row_iterator() {
        let mut b = BoardContent::new();
        let square_content = Some(SquareContent::X);
        b[3] = square_content;
        assert_eq!(3, b.rows().count());
        assert_eq!(b.rows().nth(1).unwrap().as_slice()[0], square_content);
    }

    #[test]
    fn into_string() {
        let mut b = BoardContent::new();
        b[4] = Some(SquareContent::X);
        let str = format!("{}", b);

        // This is how it should look like:
        // ┏━━┯━━┯━━┓
        // ┃  |  |  ┃
        // ┠──┼──┼──┨
        // ┃  |XX|  ┃
        // ┠──┼──┼──┨
        // ┃  |  |  ┃
        // ┗━━┷━━┷━━┛

        assert_eq!(
            "┏━━┯━━┯━━┓\n┃  |  |  ┃\n┠──┼──┼──┨\n┃  |XX|  ┃\n┠──┼──┼──┨\n┃  |  |  ┃\n┗━━┷━━┷━━┛\n",
            str
        );
    }
}
