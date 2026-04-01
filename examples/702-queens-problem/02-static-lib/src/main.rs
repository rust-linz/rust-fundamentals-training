// Approach 2: build.rs links a pre-built static library (libqueens.a).
// The library code is copied into the binary at link time; no .a file is
// needed at runtime.
//
// Compared to approach 1, the only difference is how the C code reaches
// the binary: here via a pre-built archive rather than compiled on-the-fly.
fn main() {
    queens_static_lib::run_demo();
}
