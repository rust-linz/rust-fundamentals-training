# WebAssembly Battleship

## Introduction

Rust works very well with WebAssembly. Therefore we are going to implement a version of our Battleship browser UI in which the entire game logic runs within the browser. Of course we want to reuse our existing game logic for that purpose.

## Create Package

We will follow the [*Getting Started* guide in the Rust WASM book](https://rustwasm.github.io/docs/book/game-of-life/hello-world.html) to create our new package. Run `cargo generate --git https://github.com/rustwasm/wasm-pack-template` and use *battleship_wasm* as your project name.

If you want, you can delete files that are not relevant for learning Rust (e.g. readme-files, license-files, etc.).

Use [*wasm-pack*](https://rustwasm.github.io/wasm-pack/book/) to build the code: `wasm-pack build`

## Dependencies

Overwrite the dependencies in the library's *Cargo.toml* with the code from [Cargo.toml](../999-final/battleship_wasm/Cargo.toml). Lookup all the referenced crates on [crates.io](https://crates.io/) and find out what they are used for. You do not need to understand all the details. Overview knowledge is sufficient. Note how we reference our game logic in *battleship_game_logic*.

**Note:** In this example, we do not use [*wee_alloc*](https://github.com/rustwasm/wee_alloc) for WASM size optimization.

## *lib.rs*

Change the file *lib.rs* in the *battleship_wasm* package by copying the content from [lib.rs](../999-final/battleship_wasm/src/lib.rs). Make yourself familiar with the code and/or discuss it with your Rust trainers. Note the recommended reading links at the beginning of the file.

## *web.rs*

Change the file *web.rs* in the *battleship_wasm tests* package by copying the content from [web.rs](../999-final/battleship_wasm/tests/web.rs). Make yourself familiar with the code and/or discuss it with your Rust trainers. Note the recommended reading links at the beginning of the file.

## Verify

* Use `wasm-pack build` to verify that your project compiles.
* Use `cargo clippy` to verfiy that you follow Rust coding guidelines.
* Use `wasm-pack test` to verify that all tests pass.
