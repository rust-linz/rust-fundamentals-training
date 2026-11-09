# Tic Tac Toe

## Introduction

Welcome to the Rust Tic Tac Toe sample! In this sample, you will have the opportunity to implement a simple Tic Tac Toe game using the Rust programming language. This will give you a chance to practice using Rust's powerful features, including its strong static type system and its support for traits. By the end of this sample, you will have a working Tic Tac Toe game that you can play in the console.

## Requirements

### Level 1: Square Content

Implement a Rust *enum* data type representing the content of a Tic Tac Toe square. If a square contains content, the content can be *X* or *O*.

Support converting square content values from/into the following data types:

* `u8` (X should become 1, O should become 2)
* `char` (`'X'` and `'O'`)

Converting a square content value into one of these types can never fail, so implement Rust's `From` trait for that direction. The other direction can fail (e.g. `99` is not a valid square content). The [documentation of `From`](https://doc.rust-lang.org/std/convert/trait.From.html) says that such conversions must not use `From`, so implement Rust's `TryFrom` trait with a custom error type instead.

Non-functional requirements:

* Implement your type and related functionality in a separate module.
* If you have time, write unit tests for your code.

### Level 2: Board Content

We have to store the content of our Tic Tac Toe board somewhere. Implement a *struct*  `BoardContent` for that. The struct has to store the 9 values for our game board. Choose the proper data type to store the game state yourself.

Your `BoardContent` struct should have at least the following features:

* Constructor function `new` to make it simple to create new instances of the struct.
* Offer a function `to_compact_str` that allows us to export the content of the board into a single string consisting of 9 characters. Use blank for squares that are empty. Use the `From` trait you implemented in level 1 to turn square content values into characters.
* Implement Rust's `TryFrom` trait to support converting `u8` slices with length 9 into a board content instance. Zeroes should become empty squares. Use the `TryFrom` trait you implemented in level 1 to turn other values into square content values. Return an error if the slice has the wrong length or contains an invalid value.
* Implement Rust's `From` trait to support converting a board content instance into an `u8` array with size 9. Empty squares should become 0. Use the `From` trait you implemented in level 1 to turn square content values into bytes.
* Implement Rust's `Index` and `IndexMut` traits for your board content class. Use `usize` values as indexes.

Non-functional requirements:

* Implement your type and related functionality in a separate module.
* If you have time, write unit tests for your code.

### Level 3: Board Index

Implement a struct `BoardIndex` that represents a valid index in your Tic Tac Toe board.

Your `BoardIndex` struct should have at least the following features:

* Implement constructor functions:
  * Without parameters: Creates a board index for the left upper square of the board.
  * `from_index`: Creates a board index from continuous values between 0 and 8 (0..=2 is first row, 3..=5 is second row, 6..=8 is third row)
  * `from_col_row`: Creates a board index from row and column-index (zero-based).
* Offer two functions `column` and `row` that return the corresponding column and row of the board index.
* Offer functions to navigate between board indexes. They should return a new board index.
  * `next_column` and `previous_column`
  * `next_row` and `previous_row`
  * `try_next` and `try_previous`
* Implement the `Add` and `Sub` traits to enable adding/subtracting `usize` values to board indexes.
* Implement Rust's traits for converting a board index to/from strings (*A1*..*C3*, the letter is the column, the digit is the row)
  * `FromStr`
  * `Display`
* Implement Rust's `Index` and `IndexMut` traits for your board content class. Use `BoardIndex` value as indexes.

### Level 4: Row Iterator

Working with the board row by row is a recurring task. Implement types that provide a view on the rows of a board without copying its content.

Your solution should have at least the following features:

* A struct `Row` that holds a reference to a `BoardContent` and the index of the row. Think about why this struct needs a lifetime annotation.
  * Offer a function `as_slice` that returns the squares of the row as a slice.
  * Implement Rust's `Index` trait with `usize` values as indexes so that `row[1]` returns the second square of the row.
* A struct `RowsIterator` that implements Rust's `Iterator` trait and returns the rows of a board from top to bottom.
* A trait `Rows` with the following functions. Implement it for `BoardContent`.
  * `row` returns the `Row` with a given index.
  * `rows` returns an iterator over all rows as a trait object (`Box<dyn Iterator<Item = Row<'_>> + '_>`).
* Implement Rust's `Display` trait for `BoardContent` using your iterator. Here is the expected output for a board with an *X* in the center:

```txt
┏━━┯━━┯━━┓
┃  |  |  ┃
┠──┼──┼──┨
┃  |XX|  ┃
┠──┼──┼──┨
┃  |  |  ┃
┗━━┷━━┷━━┛
```

Non-functional requirements:

* Implement your types and related functionality in a separate module.
* If you have time, write unit tests for your code.

### Level 5: Game Logic

Time to bring everything together and make the game playable.

Your solution should have at least the following features:

* A trait `SquareAccessor` with the functions `get` and `set` to read and write a square identified by a `BoardIndex`. Add a blanket implementation for all types that implement `Index<BoardIndex>` and `IndexMut<BoardIndex>`. Thanks to this trait, unit tests can replace the real board with a mock object.
* A generic struct `Game<T>` that stores the board content (`T: SquareAccessor`) and the current player.
  * Constructor function `new`. Player *X* starts the game.
  * `set` places the mark of the current player on a given square and switches to the other player. It returns a `Result` with a custom error type. Setting a square that already has a value is an error. So is a board on which the numbers of *X* and *O* differ by more than one.
  * `who_is_next` returns the current player.
  * `is_board_full` returns `true` if no empty square is left.
* A trait `WinnerDetector` with a function `get_winner` that returns the winner, if there is one. Implement it for `Game<T>`. Tip: There are only 8 winning lines (3 rows, 3 columns, 2 diagonals). Consider storing them in a constant.
* Implement Rust's `Display` trait for `Game<T>`.
* Write the `main` function. It shows the board and asks for a location until there is a winner or the board is full. Use your `FromStr` implementation to parse the input. If the input is invalid or `set` returns an error, print a message and ask again. At the end, print the winner or that the game ended in a draw.

Non-functional requirements:

* Implement your types and related functionality in a separate module.
* If you have time, write unit tests for your code. Use the [*mockall* crate](https://docs.rs/mockall/latest/mockall/) to test `Game<T>` without a real board.
