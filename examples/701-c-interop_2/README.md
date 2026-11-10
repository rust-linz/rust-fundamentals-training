# C Interop: Calling Rust from C

This sample shows how a C program calls a function written in Rust. Example [700-c-interop](../700-c-interop/) shows the opposite direction, Rust calling C. Both calculate the area of an ellipse, so you can compare them side by side.

## Structure

- [`area_calculator_rs`](area_calculator_rs/) is a Rust library compiled as a static library (`crate-type = ["staticlib"]`).
- [`area_calculator_c`](area_calculator_c/) is a C program that uses the Rust library.

## How it works

- **Exporting the function:** In [`lib.rs`](area_calculator_rs/src/lib.rs), `#[unsafe(no_mangle)] pub extern "C"` exports `area_ellipse` with the C calling convention and an unmangled name, so the C linker can find it.
- **Nullable pointers:** The function takes `Option<&Ellipse>`. This has the same ABI as a C pointer, where NULL becomes `None`. A plain `&Ellipse` would promise Rust that the pointer is never NULL, which C code cannot guarantee.
- **Generated header:** [`build.rs`](area_calculator_rs/build.rs) uses [cbindgen](https://github.com/mozilla/cbindgen) to generate `target/include/area_calculator_rs.h` from the Rust code on every build. The C code includes this header, so the header always matches the Rust API. [`cbindgen.toml`](area_calculator_rs/cbindgen.toml) configures the output.
- **Linking:** A Rust static library needs the native system libraries that Rust's standard library uses. The [justfile](area_calculator_c/justfile) passes them to `gcc`. Get the list for your platform with `cargo rustc --lib -- --print native-static-libs`.

## Run

You need `gcc` and [`just`](https://github.com/casey/just). The justfile is written for Linux.

```sh
cd area_calculator_c
just run
```
