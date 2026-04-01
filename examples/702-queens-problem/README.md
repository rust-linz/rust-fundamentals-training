# 702 — Rust C Interop: N-Queens Problem

This example demonstrates **three ways to call a C library from Rust** using the
Foreign Function Interface (FFI). The underlying algorithm — solving the N-Queens
problem — is implemented in C and called from safe Rust wrappers.

## What is the N-Queens problem?

Place *n* queens on an *n×n* chessboard so that no queen can attack any other queen
(no two queens share a row, column, or diagonal).

## Project structure

```
702-queens-problem/
├── queens_c/          C library: algorithm implementation + header + Makefile
├── 01-build-rs/       Approach 1 — build.rs compiles C via the cc crate
├── 02-static-lib/     Approach 2 — links a pre-built static library (.a)
├── 03-dynamic-lib/    Approach 3 — links a pre-built dynamic library (.dylib)
└── justfile           All build / run / test / lint commands
```

Each of the three Rust projects is a standalone Cargo package. They share
identical `src/ffi.rs` and `src/lib.rs`; the only differences are `build.rs`,
`Cargo.toml`, and the crate name referenced in `src/main.rs`.

---

## The three approaches

### Approach 1 — `build.rs` compiles C (`01-build-rs`)

The [`cc` crate](https://crates.io/crates/cc) is a build-time helper that
invokes the system C compiler, archives the object file into `$OUT_DIR/libqueens.a`,
and automatically emits the Cargo link directives.

```toml
[build-dependencies]
cc = "1"
```

**`build.rs`** simply calls:
```rust
cc::Build::new()
    .file("../queens_c/queens.c")
    .compile("queens");
```

No pre-built library is needed. This is the easiest approach when you own the C
source and want Cargo to manage the full build.

---

### Approach 2 — static library (`02-static-lib`)

A static archive (`libqueens.a`) is built with `make` and the C code is
**copied into** the Rust binary at link time.

**`build.rs`** runs `make` and emits two Cargo directives:
```
cargo:rustc-link-search=native=../queens_c   # -L: where to look
cargo:rustc-link-lib=static=queens           # -l: what to link
```

The resulting binary is self-contained — `libqueens.a` is only needed at
build time, not at runtime.

---

### Approach 3 — dynamic library (`03-dynamic-lib`)

A dynamic library (`libqueens.dylib`) is built with `make`. The Rust binary
records only a **reference** to the library; the OS dynamic linker loads it at
runtime.

**`build.rs`** emits:
```
cargo:rustc-link-search=native=../queens_c
cargo:rustc-link-lib=dylib=queens             # dynamic link
```

Because the library is loaded at runtime, you must tell the macOS dynamic linker
where to find it:

```bash
DYLD_LIBRARY_PATH=./queens_c ./target/debug/queens
```

The `justfile` sets this automatically for `just run-03` and `just test-03`.

---

## The FFI boundary

### `src/ffi.rs` — raw C bindings

```
#[repr(C)]           forces C-compatible struct layout (same field order & padding)
unsafe extern "C"    declares functions using the C calling convention
```

In **Rust 2024** `extern` blocks must be prefixed with `unsafe`. Functions that
take only plain integers (no pointers) and handle all inputs safely can be
individually marked `safe`, allowing them to be called without an `unsafe` block.

The non-trivial data structure crossing the boundary is:

```rust
#[repr(C)]
pub struct RawQueensBoard {
    pub n: c_int,
    pub positions: [c_int; 16],   // mirrors  int positions[QUEENS_MAX_N]  in C
}
```

### `src/lib.rs` — safe Rust wrappers

The wrappers translate C idioms into Rust idioms:

| C convention | Rust equivalent |
|---|---|
| Return 0/1 flag | `Option<T>` / `bool` |
| Caller-provided output array | `Vec<T>` allocated by the wrapper |
| Raw `*mut` pointer | reference to a local variable / slice |
| `int` counts | `usize` |
| Struct with `n` + fixed array | `Vec<usize>` (length *is* `n`, no redundant field) |

Every `unsafe { }` block is accompanied by a `// SAFETY:` comment explaining
why the call is sound. All other code in `lib.rs` is safe Rust.

---

## Running the examples

You need [just](https://github.com/casey/just) and a C compiler (`gcc` / `clang`).

```bash
# Run all three examples (builds C libs as needed)
just all

# Or run each approach individually
just run-01    # approach 1: cc crate (no pre-build needed)
just run-02    # approach 2: static library
just run-03    # approach 3: dynamic library

# Run tests
just test      # all three projects
just test-01   # approach 1 only
just test-02   # approach 2 only
just test-03   # approach 3 only

# Lint (clippy -D warnings)
just lint      # all three
just lint-01   # approach 1 only

# Build C libraries without running Rust
just build-static    # produces queens_c/libqueens.a
just build-dynamic   # produces queens_c/libqueens.dylib

# Clean all build artifacts
just clean
```

### Why does `run-03` need `DYLD_LIBRARY_PATH`?

With dynamic linking, the OS loader must find `libqueens.dylib` at runtime.
On macOS it searches standard system paths by default. Since our library lives
in `queens_c/` (a development directory), we set `DYLD_LIBRARY_PATH` to point
there. The `justfile` does this automatically; you only need to worry about it
if you run the binary directly.

---

## Key concepts for Rust developers new to C interop

| Concept | Where to look |
|---|---|
| `#[repr(C)]` and why it matters | `src/ffi.rs` — struct declaration |
| `unsafe extern "C"` block | `src/ffi.rs` — function declarations |
| `safe fn` in Rust 2024 | `src/ffi.rs` — `queens_count` |
| `// SAFETY:` comments | `src/lib.rs` — every `unsafe {}` block |
| `Vec` as a C output buffer | `src/lib.rs` — `find_all()` |
| `std::ffi::c_int` | `src/ffi.rs`, `src/lib.rs` |
| `cc` crate in `build.rs` | `01-build-rs/build.rs` |
| `cargo:rustc-link-*` directives | `02-static-lib/build.rs`, `03-dynamic-lib/build.rs` |
| Static vs dynamic linking | `02-static-lib/build.rs`, `03-dynamic-lib/build.rs` |
| `DYLD_LIBRARY_PATH` at runtime | `justfile` — `run-03`, `test-03` |
