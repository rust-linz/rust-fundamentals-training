# Board Index

## Introduction

Our board content is stored in a one-dimensional array of 100 elements. However, in a Battleship game, you shoot by specifying column and row. In this exercise, we want to implement a data type that implements the indexing logic.

In a Battleship game, you frequently need to specify a *range* of squares (e.g. a ship reaching from square A3 to square A6). In this exercise, we will implement a data type for that use case, too.

Of course, you will again learn about a lot of important traits during this exercise. Additionally, we will look at more advanced testing strategies (in particular data-driven tests with the *rstest* crate).

## *board_index.rs*

Add the files *board_index.rs* and *board_index_range.rs* to the *battleship_game_logic* package. Copy the contents from [board_index.rs](../999-final/battleship_game_logic/src/board_index.rs) and [board_index_range.rs](../999-final/battleship_game_logic/src/board_index_range.rs) or implement it yourself based on the included unit tests. Make yourself familiar with the code and/or discuss it with your Rust trainers. Note the recommended reading links at the beginning of the files.

Don't forget to add the new module to *lib.rs*.

## Verify

* Use `cargo build` to verify that your project compiles.
* Use `cargo clippy` to verfiy that you follow Rust coding guidelines.
* Use `cargo test` to verify that all tests pass.

## Discussions

Extend the CLI package created before. Add a subcommand that uses logic in `BoardIndex` and/or `BoardIndexRangeInclusive` and prints the result on the screen.
