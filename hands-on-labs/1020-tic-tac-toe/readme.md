# Tic Tac Toe

## Introduction

Welcome to the Rust Tic Tac Toe sample! In this sample, you will have the opportunity to implement a simple Tic Tac Toe game using the Rust programming language. This will give you a chance to practice using Rust's powerful features, including its strong static type system and its support for traits. By the end of this sample, you will have a working Tic Tac Toe game that you can play in the console.

## Requirements

### Level 1: Square Content

Implement a Rust *enum* data type representing the content of a Tic Tac Toe square. If a square contains content, the content can be *X* or *O*.

Implement Rust's `From` trait to support converting square content values from/into the following data types:

* `u8` (X should become 1, O should become 2)
* `char` (`'X'` and `'O'`)

Non-functional requirements:

* Implement your type and related functionality in a separate module.
* If you have time, write unit tests for your code.

### Level 2: Board Content

We have to store the content of our Tic Tac Toe board somewhere. Implement a *struct*  `BoardContent` for that. The struct has to store the 9 values for our game board. Choose the proper data type to store the game state yourself.

Your `BoardContent` struct should have at least the following features:

* Constructor function `new` to make it simple to create new instances of the struct.
* Offer a function `to_compact_str` that allows us to export the content of the board into a single string consisting of 9 characters. Use blank for squares that are empty. Use the `From` trait you implemented in level 1 to turn square content values into characters.
* Implement Rust's `TryFrom` trait to support converting `u8` slices with length 9 into a board content instance. Zeroes should become empty squares. Use the `From` trait you implemented in level 1 to turn other values into square content values.
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
* Implement Rust's traits for converting a board index to/from strings (*A1*..*C3*)
  * `FromStr`
  * `Display`
* Implement Rust's `Index` and `IndexMut` traits for your board content class. Use `BoardIndex` value as indexes.

### Level 4: Row Iterator

TBD

### Level 5: Game Logic

TBD
