/// Build script for the `03-dynamic-lib-linux` approach.
///
/// Links against a **pre-built shared library** (`libqueens.so` on Linux).
/// See `03-dynamic-lib-macos` for the macOS variant (`libqueens.dylib`).
///
/// The key difference from the static approach is `cargo:rustc-link-lib=dylib=`:
///
/// - `static=` → the library's code is **copied into** the binary at link time.
///   The resulting binary is self-contained; `libqueens.a` is only needed at
///   build time.
///
/// - `dylib=` → the binary records only a **reference** to `libqueens.so`.
///   The OS dynamic linker (`ld.so`) loads the actual library at **runtime**.
///   This means `libqueens.so` must be discoverable at runtime, e.g. via
///   `LD_LIBRARY_PATH` (dev), an `rpath` embedded in the binary, or by
///   installing it into a system library directory and running `ldconfig`.
///
/// The `justfile` at the project root sets `LD_LIBRARY_PATH` automatically
/// for `just run-03` and `just test-03`.
use std::env;
use std::process::Command;

fn main() {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    assert!(
        target_os == "linux",
        "03-dynamic-lib-linux only works on Linux (target: {target_os}); use 03-dynamic-lib-macos instead"
    );

    let status = Command::new("make")
        .arg("libqueens.so")
        .current_dir("../queens_c")
        .status()
        .expect("failed to run make; is `make` installed?");
    assert!(status.success(), "make failed to build libqueens.so");

    println!("cargo:rustc-link-search=native=../queens_c");
    println!("cargo:rustc-link-lib=dylib=queens");
    println!("cargo:rerun-if-changed=../queens_c/queens.c");
    println!("cargo:rerun-if-changed=../queens_c/queens.h");
}
