/// Build script for the `03-dynamic-lib` approach.
///
/// Links against a **pre-built dynamic library**. Dynamic libraries are
/// platform-specific, so the script picks the right file for the target OS:
///
/// - Linux: `libqueens.so`, found at runtime via `LD_LIBRARY_PATH`
/// - macOS: `libqueens.dylib`, found at runtime via `DYLD_LIBRARY_PATH`
///
/// The key difference from the static approach is `cargo:rustc-link-lib=dylib=`:
///
/// - `static=` → the library's code is **copied into** the binary at link time.
///   The resulting binary is self-contained; `libqueens.a` is only needed at
///   build time.
///
/// - `dylib=` → the binary records only a **reference** to the library.
///   The OS dynamic linker loads the actual library at **runtime**.
///   This means the library must be discoverable at runtime, e.g. via
///   `LD_LIBRARY_PATH` / `DYLD_LIBRARY_PATH` (dev), an `rpath` embedded in the
///   binary, or by installing it into a system library directory.
///
/// The `justfile` at the project root sets the environment variable
/// automatically for `just run-03` and `just test-03`.
use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let lib_file = match target_os.as_str() {
        "linux" => "libqueens.so",
        "macos" => "libqueens.dylib",
        _ => panic!("03-dynamic-lib supports Linux and macOS only (target: {target_os})"),
    };

    // Use an absolute path. The link search path is passed on to rustc, so a
    // relative path would depend on rustc's working directory.
    let queens_c = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("../queens_c");

    let status = Command::new("make")
        .arg(lib_file)
        .current_dir(&queens_c)
        .status()
        .expect("failed to run make; is `make` installed?");
    assert!(status.success(), "make failed to build {lib_file}");

    println!("cargo:rustc-link-search=native={}", queens_c.display());
    println!("cargo:rustc-link-lib=dylib=queens");
    for file in ["queens.c", "queens.h", "Makefile"] {
        println!("cargo:rerun-if-changed={}", queens_c.join(file).display());
    }
}
