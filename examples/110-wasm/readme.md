# Rust Wasm Sample

## Important Links

* Interesting blob posts
  * Lin Clark: [Making WebAssembly better for Rust & for all languages](https://hacks.mozilla.org/2018/03/making-webassembly-better-for-rust-for-all-languages/) (March 2018)
  * Alex Crichton: [JavaScript to Rust and Back Again: A *wasm-bindgen* Tale](https://hacks.mozilla.org/2018/04/javascript-to-rust-and-back-again-a-wasm-bindgen-tale/) (April 2018)
  * Ashley Williams: [Hello *wasm-pack*!](https://hacks.mozilla.org/2018/04/hello-wasm-pack/) (April 2018)
  * Lin Clark: [Calls between JavaScript and WebAssembly are finally fast 🎉](https://hacks.mozilla.org/2018/10/calls-between-javascript-and-webassembly-are-finally-fast-%f0%9f%8e%89/) (October 2018)
* Documentation
  * [The *wasm-bindgen* Guide](https://rustwasm.github.io/docs/wasm-bindgen/introduction.html)
  * [Hello *wasm-pack*!](https://rustwasm.github.io/docs/wasm-pack/)
* Crates
  * [*wasm-bindgen*](https://crates.io/crates/wasm-bindgen)
  * [*wasm-bindgen-test*](https://crates.io/crates/wasm-bindgen-test)
  * [*js-sys*](https://crates.io/crates/js-sys)
  * [*serde-wasm-bindgen*](https://crates.io/crates/serde-wasm-bindgen)
  * [*web_sys*](https://crates.io/crates/web_sys)

## Storyboard

We will follow the [*Getting Started* guide in the Rust Wasm book](https://rustwasm.github.io/docs/book/game-of-life/hello-world.html) to create our new package. Run `cargo generate --git https://github.com/rustwasm/wasm-pack-template` and use *wasm_examples* as your project name.

If you want, you can delete files that are not relevant for learning Rust (e.g. readme-files, license-files, etc.).

Use [*wasm-pack*](https://rustwasm.github.io/wasm-pack/book/) to build the code: `wasm-pack build`

Use `wasm-pack test --chrome` to verify that all tests pass.

Discuss the anatomy of the generated project. After this, copy the sample code from this folder into your project.

The sample code in [*lib.rs*](src/lib.rs) and [*index.ts*](www/src/index.ts) demonstrates various aspects of Rust Wasm. Do a code walkthrough region-by-region. In longer workshops and trainings you can also write the code region-by-region and discuss it in details.
