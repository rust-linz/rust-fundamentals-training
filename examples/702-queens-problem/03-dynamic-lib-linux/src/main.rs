// Approach 3 (Linux): build.rs links a pre-built shared library (libqueens.so).
// Unlike static linking, the binary only records a *reference* to the .so;
// the OS dynamic linker loads it at runtime. This requires LD_LIBRARY_PATH
// to point to the directory containing libqueens.so.
// The justfile handles this automatically for `just run-03` and `just test-03`.
fn main() {
    queens_dynamic_lib::run_demo();
}
