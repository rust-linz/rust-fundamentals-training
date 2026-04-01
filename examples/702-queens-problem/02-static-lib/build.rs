/// Build script for the `02-static-lib` approach.
///
/// Links against a **pre-built** static library (`libqueens.a`).
///
/// # Why copy to OUT_DIR?
///
/// On macOS, `ld` prefers a `.dylib` over a `.a` when both are present in the
/// same search path directory, even when `-Bstatic` is requested. If the user
/// has previously run `just run-03` (which builds `libqueens.dylib` into
/// `queens_c/`), the macOS linker would silently pick the dynamic library.
///
/// The fix is to copy `libqueens.a` into `$OUT_DIR` — a directory that Cargo
/// creates fresh for each crate and that will never contain a `.dylib`. By
/// pointing the link search path to `$OUT_DIR`, the linker has no choice but
/// to use the static archive.
///
/// # Cargo link directives
///
/// - `cargo:rustc-link-search=native=<path>`
///   Adds `<path>` to the linker's library search path (-L in gcc terms).
///
/// - `cargo:rustc-link-lib=static=queens`
///   Links `libqueens.a` statically. Library code is copied into the binary
///   at link time; the `.a` file is not needed at runtime.
///
/// - `cargo:rerun-if-changed=<file>`
///   Tells Cargo to re-run this script only when the listed file changes.
use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    let status = Command::new("make")
        .arg("libqueens.a")
        .current_dir("../queens_c")
        .status()
        .expect("failed to run make; is `make` installed?");
    assert!(status.success(), "make failed to build libqueens.a");

    // Copy the static library into OUT_DIR so the linker search path contains
    // only the .a, not a potentially co-located .dylib (see doc comment above).
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set by Cargo");
    fs::copy(
        Path::new("../queens_c/libqueens.a"),
        Path::new(&out_dir).join("libqueens.a"),
    )
    .expect("failed to copy libqueens.a to OUT_DIR");

    println!("cargo:rustc-link-search=native={out_dir}");
    println!("cargo:rustc-link-lib=static=queens");
    println!("cargo:rerun-if-changed=../queens_c/queens.c");
    println!("cargo:rerun-if-changed=../queens_c/queens.h");
}
