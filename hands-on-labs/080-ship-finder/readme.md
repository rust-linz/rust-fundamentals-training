# Ship Finder

## Introduction

The next step for our game logic is code that can find out whether there is a ship on a given location. Additionally, it can recognize if the ship's location is already fully known (i.e. now unknown cells adjecent to the ship's beginning or end). This functionality is super important when e.g. building a computer player for Battleship.

## *ship_finder.rs*

Add the file *ship_finder.rs* to the *battleship_game_logic* package. Copy the content from [ship_finder.rs](../999-final/battleship_game_logic/src/ship_finder.rs) or implement it yourself based on the included unit tests. Make yourself familiar with the code and/or discuss it with your Rust trainers. Note the recommended reading links at the beginning of the file.

Don't forget to add the new modules to *lib.rs*.

## Verify

* Use `cargo build` to verify that your project compiles.
* Use `cargo clippy` to verfiy that you follow Rust coding guidelines.
* Use `cargo test` to verify that all tests pass.
