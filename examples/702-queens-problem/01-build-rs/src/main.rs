// Approach 1: build.rs compiles the C source directly using the `cc` crate.
// No pre-built library is required.
//
// This file is intentionally minimal — all interesting logic lives in lib.rs
// so the three example projects (01, 02, 03) differ only in this one line.
fn main() {
    queens_build_rs::run_demo();
}
