# Fillable Board

## Introduction

So far, we have implemented useful data structures to hold the content of the board. Next, we want to focus on the game logic for placing ships. The usual Battleship rules apply: Each ship occupies a number of *consecutive* squares on the grid, arranged *either horizontally or vertically* (not diagnoally). There *must not be directly adjacent* to each other (i.e. at least one water cell has to be between ships).

The following table contains the ships that are placed on each player's grid:

| Class of ship | Size |
| ------------- | ---: |
| Carrier       |    5 |
| Battleship    |    4 |
| Cruiser       |    3 |
| Submarine     |    3 |
| Destroyer     |    2 |

## *fillable_board.rs*

Add the files *fillable_board.rs* and *board_filler.rs* to the *battleship_game_logic* package. Copy the content from [fillable_board.rs](../999-final/battleship_game_logic/src/fillable_board.rs) and [board_filler.rs](../999-final/battleship_game_logic/src/board_filler.rs) or implement it yourself based on the included unit tests. Make yourself familiar with the code and/or discuss it with your Rust trainers. Note the recommended reading links at the beginning of the files.

Don't forget to add the new modules to *lib.rs*.

Pay particular attention to the unit tests in this exercise. For the first time, we use *mock objects* using the popular *mockall* crate. Make yourself familiar with this crate.

## Verify

* Use `cargo build` to verify that your project compiles.
* Use `cargo clippy` to verfiy that you follow Rust coding guidelines.
* Use `cargo test` to verify that all tests pass.

## Discussion

Can you remember that we commented out a line of code in our CLI project? It was related to filling the board. Now that we have that logic, you can add the line of code and test whether filling the board randomly works as expected.
