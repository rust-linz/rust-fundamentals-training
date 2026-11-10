# C Interop: Calling C from Rust

This sample shows how a Rust program calls a function from a C library. Example [701-c-interop_2](../701-c-interop_2/) shows the opposite direction, C calling Rust. Both calculate the area of an ellipse, so you can compare them side by side.

## Structure

- [`area_calculator_c`](area_calculator_c/) is a small C library. The [Makefile](area_calculator_c/Makefile) compiles it into the static library `libareacalc.a`.
- [`area_calculator_rs`](area_calculator_rs/) is a Rust binary that uses the C library.

## How it works

- **Build script:** [`build.rs`](area_calculator_rs/build.rs) runs `make` for the C library and tells Cargo where to find `libareacalc.a` and to link it. It also tells Cargo to rerun when the C sources change, because they are outside the Rust package.
- **Matching the C types:** In [`main.rs`](area_calculator_rs/src/main.rs), the `Ellipse` struct mirrors the C struct `ellipse_t`. `#[repr(C)]` guarantees the same memory layout.
- **Declaring the C function:** An `unsafe extern "C"` block declares `area_ellipse` with raw pointers, exactly like its C signature.
- **Safe wrapper:** A safe Rust function `area_ellipse(&Ellipse) -> f64` contains the only `unsafe` block. The rest of the program uses this normal Rust API. Keeping `unsafe` in small, documented wrappers is the most important pattern for FFI code.

In real projects, you would usually compile C code with the [`cc`](https://crates.io/crates/cc) crate instead of calling `make`. It builds into Cargo's output directory and works on all platforms. This sample uses `make` to show what happens under the hood.

## Run

You need `gcc` and `make`.

```sh
cd area_calculator_rs
cargo run
```
