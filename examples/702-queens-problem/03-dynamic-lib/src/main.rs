// Approach 3: build.rs links a pre-built dynamic library (libqueens.so on
// Linux, libqueens.dylib on macOS). Unlike static linking, the binary only
// records a *reference* to the library; the OS dynamic linker loads it at
// runtime. This requires LD_LIBRARY_PATH (Linux) or DYLD_LIBRARY_PATH (macOS)
// to point to the directory containing the library.
// The justfile handles this automatically for `just run-03` and `just test-03`.
fn main() {
    queens_dynamic_lib::run_demo();
}
