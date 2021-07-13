# Generic Board Content

## Introduction

We want to implement an advanced version of our Battleship board data structure where the square content isn't fixed by specified using a type parameter.

In addition to making our board generic, we also work with [iterators](https://doc.rust-lang.org/book/ch13-02-iterators.html) a lot in this exercise. Pay particular attention to the corresponding code parts.

## *generic_board_content.rs* and *row.rs*

Add the files *generic_board_content.rs* and *row.rs* to the *battleship_game_logic* package. Copy the content from [generic_board_content.rs](../999-final/battleship_game_logic/src/generic_board_content.rs) and [row.rs](../999-final/battleship_game_logic/src/row.rs) or implement it yourself based on the included unit tests. Make yourself familiar with the code and/or discuss it with your Rust trainers. Note the recommended reading links at the beginning of the files.

Don't forget to add the new modules to *lib.rs*.

## Verify

* Use `cargo build` to verify that your project compiles.
* Use `cargo clippy` to verfiy that you follow Rust coding guidelines.
* Use `cargo test` to verify that all tests pass.

## Debug

Add the file *.vscode/launch.json* in the root folder of your workspace. Copy the content from [.vscode/launch.json](../999-final/.vscode/launch.json). This will make it possible to debug unit tests with [*CodeLLDB*](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb). Pick a unit test, set a breakpoint, and try to debug the test code.

Practice debugging throughout the following exercises. You can take the content of [.vscode/launch.json](../999-final/.vscode/launch.json) as a basis for configuring the debugger for other packages.

## Discussions

Analyze how the iterator implementation on `Row` is used in the implementation of the `Display` trait for `GenericBoardContent`.
