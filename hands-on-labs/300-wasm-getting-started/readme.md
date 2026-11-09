# WebAssembly Battleship

## Introduction

Rust works very well with WebAssembly. Therefore we are going to implement a version of our Battleship browser UI in which the entire game logic runs within the browser. Of course we want to reuse our existing game logic for that purpose.

## Create Package

Create a new library using Cargo: `cargo new --lib battleship_wasm`. Add the new library to the previously created workspace.

We use [*wasm-pack*](https://drager.github.io/wasm-pack/) to build the code. Install it if you have not done so already.

## Dependencies

Overwrite the library's *Cargo.toml* with the content from [Cargo.toml](../999-final/battleship_wasm/Cargo.toml). Lookup all the referenced crates on [crates.io](https://crates.io/) and find out what they are used for. You do not need to understand all the details. Overview knowledge is sufficient. Note how we reference our game logic in *battleship_game_logic*.

**Tip:** You might ask yourself why we reference *getrandom* with the feature *wasm_js*. Our game logic needs random numbers, and in the browser they have to come from the Web Crypto API ([read more](https://docs.rs/getrandom/latest/getrandom/)). The game logic should stay independent of the browser, so we enable this feature only in the WASM package.

## *lib.rs* and *utils.rs*

Change the file *lib.rs* and add the file *utils.rs* in the *battleship_wasm* package by copying the content from [lib.rs](../999-final/battleship_wasm/src/lib.rs) and [utils.rs](../999-final/battleship_wasm/src/utils.rs). Make yourself familiar with the code and/or discuss it with your Rust trainers. Note the recommended reading links at the beginning of the file.

## *web.rs*

Add the file *tests/web.rs* to the *battleship_wasm* package by copying the content from [web.rs](../999-final/battleship_wasm/tests/web.rs). Make yourself familiar with the code and/or discuss it with your Rust trainers.

## Verify

Run the following commands in the *battleship_wasm* folder:

* Use `wasm-pack build` to verify that your project compiles.
* Use `cargo clippy` to verfiy that you follow Rust coding guidelines.
* Use `wasm-pack test --headless --chrome` to verify that all tests pass. Use `--firefox` instead of `--chrome` if you prefer Firefox.
