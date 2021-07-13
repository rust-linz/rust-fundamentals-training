# Getting Started

## Cargo Workspace

Start with an empty folder.

We are going to put all our Rust packages in a common [*Cargo Workspace*](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html). For that, create a [*Cargo.toml*](https://doc.rust-lang.org/cargo/reference/manifest.html) manifest file. Copy the content from [Cargo.toml](../999-final/Cargo.toml). Obviously, this is how the file will look like at the end of our hands-on-lab. You have to start with an empty `members`-list and add the folder names of the packages we are going to create step by step.

## Game Logic

Create a new library using Cargo: `cargo new --lib battleship_game_logic`. Add the new library to the previously created workspace.

Use `cargo build` to verify that your project compiles.

## Dependencies

Copy the dependencies into the library's *Cargo.toml* from [Cargo.toml](../999-final/battleship_game_logic/Cargo.toml). Lookup all the referenced crates on [crates.io](https://crates.io/) and find out what they are used for. You do not need to understand all the details. Overview knowledge is sufficient.

**Tips:**

* Make yourself familiar with [how to specify dependencies](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html).
* Make yourself familiar with [Cargo dependency features](https://doc.rust-lang.org/cargo/reference/features.html#dependency-features).
* You might ask yourself why we reference *getrandom* with the feature *js*. The reason is that we need WASM support for a later sample ([read more](https://docs.rs/getrandom/0.2.3/getrandom/#webassembly-support)).
