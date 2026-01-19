# Rust Wasm Sample

## Important Links

* Documentation
  * [The *wasm-bindgen* Guide](https://wasm-bindgen.github.io/wasm-bindgen/)
  * [Hello *wasm-pack*!](https://drager.github.io/wasm-pack/)
* Crates
  * [*wasm-bindgen*](https://crates.io/crates/wasm-bindgen)
  * [*wasm-bindgen-test*](https://crates.io/crates/wasm-bindgen-test)
  * [*js-sys*](https://crates.io/crates/js-sys)
  * [*serde-wasm-bindgen*](https://crates.io/crates/serde-wasm-bindgen)
  * [*web_sys*](https://crates.io/crates/web_sys)

## Storyboard

We will follow the [*Hell, World!*](https://wasm-bindgen.github.io/wasm-bindgen/examples/hello-world.html) to create our new package.

Use *wasm-pack* to build the code: `wasm-pack build`

Use `wasm-pack test --chrome` to verify that all tests pass.

The sample code in [*lib.rs*](src/lib.rs) and [*index.ts*](www/src/index.ts) demonstrates various aspects of Rust Wasm. Do a code walkthrough region-by-region. In longer workshops and trainings you can also write the code region-by-region and discuss it in details.
