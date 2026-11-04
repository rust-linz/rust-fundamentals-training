// Approach 3: build.rs links a pre-built dynamic library (libqueens.dylib).
// Unlike static linking, the binary only records a *reference* to the dylib;
// the OS dynamic linker loads it at runtime. This requires DYLD_LIBRARY_PATH
// (macOS) to point to the directory containing libqueens.dylib.
// The justfile handles this automatically for `just run-03` and `just test-03`.
fn main() {
    queens_dynamic_lib::run_demo();
}
