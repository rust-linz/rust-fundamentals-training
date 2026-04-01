/// Build script for the `01-build-rs` approach.
///
/// The `cc` crate is a build-time helper that:
///   1. Invokes the system C compiler (gcc/clang) to compile `queens.c`.
///   2. Archives the object file into `$OUT_DIR/libqueens.a`.
///   3. Automatically emits the Cargo link directives:
///      `cargo:rustc-link-lib=static=queens` and
///      `cargo:rustc-link-search=native=$OUT_DIR`
///
/// No pre-built library is needed — everything happens during `cargo build`.
/// This is the simplest approach when you own (or can distribute) the C source.
fn main() {
    cc::Build::new()
        .file("../queens_c/queens.c")
        .include("../queens_c")
        .opt_level(2)
        .warnings(true)
        .compile("queens");

    // Tell Cargo to rerun this build script only when the C sources change,
    // not on every `cargo build` invocation.
    println!("cargo:rerun-if-changed=../queens_c/queens.c");
    println!("cargo:rerun-if-changed=../queens_c/queens.h");
}
