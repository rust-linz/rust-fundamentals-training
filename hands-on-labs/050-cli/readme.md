# Command Line Interface

## Introduction

We have spent some time creating a library with game logic. Now we want to create a first UI. To keep it simple, we start with a command-line interface (CLI).

## CLI Binary

Create a new binary target using Cargo: `cargo new --bin console_game`. Add the new package to the previously created workspace.

Use `cargo build` to verify that your project compiles.

## Dependencies

Copy the dependencies into the library's *Cargo.toml* from [Cargo.toml](../999-final/console_game/Cargo.toml). Lookup all the referenced crates on [crates.io](https://crates.io/) and find out what they are used for. You do not need to understand all the details. Overview knowledge is sufficient. Note how we reference our game logic in *battleship_game_logic*.

## *main.rs*

Change the file *main.rs* in the *console_game* package by copying the content from [main.rs](../999-final/console_game/src/main.rs). Make yourself familiar with the code and/or discuss it with your Rust trainers. Note the recommended reading links at the beginning of the files.

Note that [main.rs](../999-final/console_game/src/main.rs) contains code to randomly fill the Battleship board. We have not implemented this logic yet. Therefore, comment out the corresponding line of code. You can add it later once we covered filling the board.

## Verify

* Use `cargo build` to verify that your project compiles.
* Use `cargo clippy` to verfiy that you follow Rust coding guidelines.
* Use `cargo run --bin console_game` to run the CLI. Try different command line parameters.

## Discussions

What is the difference between the `Debug` and the `Display` trait? `SquareContent` does not implement the `Display` trait. Try to implement it and try it in the CLI.

What other CLI subcommands could you think of? Try to implement an additional subcommand or command flag.
