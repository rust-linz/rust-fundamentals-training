/// Build script for the `03-dynamic-lib-macos` approach.
///
/// Links against a **pre-built dynamic library** (`libqueens.dylib` on macOS).
/// See `03-dynamic-lib-linux` for the Linux variant (`libqueens.so`).
///
/// The key difference from the static approach is `cargo:rustc-link-lib=dylib=`:
///
/// - `static=` → the library's code is **copied into** the binary at link time.
///   The resulting binary is self-contained; `libqueens.a` is only needed at
///   build time.
///
/// - `dylib=` → the binary records only a **reference** to `libqueens.dylib`.
///   The OS dynamic linker loads the actual library at **runtime**.
///   This means `libqueens.dylib` must be discoverable at runtime, e.g. via
///   `DYLD_LIBRARY_PATH` (dev) or `@rpath` (distribution).
///
/// The `justfile` at the project root sets `DYLD_LIBRARY_PATH` automatically
/// for `just run-03` and `just test-03`.
use std::env;
use std::process::Command;

fn main() {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    assert!(
        target_os == "macos",
        "03-dynamic-lib-macos only works on macOS (target: {target_os}); use 03-dynamic-lib-linux instead"
    );

    let status = Command::new("make")
        .arg("libqueens.dylib")
        .current_dir("../queens_c")
        .status()
        .expect("failed to run make; is `make` installed?");
    assert!(status.success(), "make failed to build libqueens.dylib");

    println!("cargo:rustc-link-search=native=../queens_c");
    println!("cargo:rustc-link-lib=dylib=queens");
    println!("cargo:rerun-if-changed=../queens_c/queens.c");
    println!("cargo:rerun-if-changed=../queens_c/queens.h");
}
