use std::ops::Index;

use crate::{BoardIndex, SquareContent};

struct _Shot {
    location: BoardIndex,
    result: SquareContent,
}

struct _SinglePlayerGame {
    log: Vec::<_Shot>,
    board: dyn Index<BoardIndex, Output = SquareContent>,
}