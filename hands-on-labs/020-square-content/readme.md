# Square Content

## Introduction

The classical Battleship game is played on boards with 10x10=100 squares. Our first task is to create a data structure representing the content of a single square.

## *square_content.rs*

Add the file *square_content.rs* to the *battleship_game_logic* package. Copy the content from [square_content.rs](../999-final/battleship_game_logic/src/square_content.rs) or implement it yourself based on the included unit tests. Make yourself familiar with the code and/or discuss it with your Rust trainers. Note the recommended reading links at the beginning of the file.

## *lib.rs*

In our sample, *lib.rs* will not contain any logic. It will just re-export public declarations from other modules. Take a look at [lib.rs](../999-final/battleship_game_logic/src/lib.rs) and copy the code accordingly. Obviously, this is how the file will look like at the end of our hands-on-lab. You have to start with just one module (`square_content`) and add the modules we are going to create step by step.

## Verify

* Use `cargo build` to verify that your project compiles.
* Use `cargo clippy` to verfiy that you follow Rust coding guidelines.
* Use `cargo test` to verify that all tests pass.
