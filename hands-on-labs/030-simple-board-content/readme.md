# Simple Board Content

## Introduction

We want to implement a data structure that can hold the content of a 10x10 Battleship board. This will be a two-step process. In this exercise, we create a simple version for the board. Later on we will use a more advanced board implementation using generics.

## *simple_board_content.rs*

Add the file *simple_board_content.rs* to the *battleship_game_logic* package. Copy the content from [simple_board_content.rs](../999-final/battleship_game_logic/src/simple_board_content.rs) or implement it yourself based on the included unit tests. Make yourself familiar with the code and/or discuss it with your Rust trainers. Note the recommended reading links at the beginning of the file.

Don't forget to add the new module to *lib.rs*.

Note: The size of the board is based on constants defined in [*lib.rs*](../999-final/battleship_game_logic/src/lib.rs). This has been done for demo purposes only. The algorithms in this package are not capable of handling a board side length other than 10. Such a feature would make algorithms unnecessarily compilicated, so we did not do it. After all, our goal is to learn and practice Rust, not to build the world's greatest Battleship game.

## Verify

* Use `cargo build` to verify that your project compiles.
* Use `cargo clippy` to verfiy that you follow Rust coding guidelines.
* Use `cargo test` to verify that all tests pass.

## Discussions

This hands-on-lab uses a [fixed-size array](https://doc.rust-lang.org/std/primitive.array.html) to store the board's content. Discuss in groups what other Rust data structure you could use to implement the Battleship board.
