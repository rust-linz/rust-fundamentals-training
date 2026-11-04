//! Raw FFI (Foreign Function Interface) bindings to the C queens solver.
//!
//! # What lives here
//!
//! This module is the single point where Rust and C meet. It contains:
//! - C-compatible types (`#[repr(C)]` structs), and
//! - `extern "C"` function declarations.
//!
//! **Do not use these directly** from application code.
//! Use the safe wrappers in the crate root (`lib.rs`) instead.
//! Keeping all unsafe declarations in one place makes it easy for reviewers
//! to audit the FFI surface.
//!
//! # Key attributes explained
//!
//! ## `#[repr(C)]`
//! Rust normally arranges struct fields in whatever order maximises
//! performance (it may reorder or add padding freely). `#[repr(C)]` forces
//! the **same field layout as a C compiler** would produce — same order,
//! same alignment, same size — which is required when passing structs across
//! the FFI boundary.
//!
//! ## `unsafe extern "C" { … }`
//! The `extern "C"` block tells Rust two things:
//! 1. These functions *exist somewhere* (the linker will resolve them).
//! 2. They use the **C calling convention** (how arguments are passed in
//!    registers/on the stack, who cleans up, etc.).
//!
//! The `"C"` string is the **ABI name**, not the programming language.
//! Other valid ABIs include `"system"` (Windows API) or `"stdcall"`.
//!
//! In Rust 2024 the block must be prefixed with `unsafe`, reflecting that
//! calling most C functions is inherently unsafe. Individual functions can
//! be marked `safe` when the programmer has verified they are sound to call
//! from safe Rust code (see `queens_count` below).

use std::ffi::c_int;

// Note: `c_int` from `std::ffi` is the correct way to refer to C's `int`.
// It is *not* simply `i32` on all platforms (though it is on the common
// 32-/64-bit platforms supported today). Using `c_int` keeps the code
// correct and self-documenting.

/// Maximum board size supported by the C library.
pub const QUEENS_MAX_N: usize = 16;

/// Mirror of `queens_board_t` from `queens.h`.
///
/// `#[repr(C)]` ensures identical memory layout to the C struct so that
/// a pointer to this type can be passed directly to C without marshalling.
///
/// The `positions` field maps to `int positions[QUEENS_MAX_N]` in C —
/// a fixed-size array embedded in the struct. This is the non-trivial
/// data structure that crosses the FFI boundary.
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct RawQueensBoard {
    /// Board size (number of queens).
    pub n: c_int,
    /// `positions[row]` = 0-based column of the queen in `row`.
    /// Only elements `[0..n]` are valid after a successful solve call.
    pub positions: [c_int; QUEENS_MAX_N],
}

// Why `unsafe extern "C"`?
//
// Every call to a function declared here is unsafe because Rust cannot verify:
//   • The function actually exists with exactly this signature (wrong signature
//     → undefined behaviour at runtime).
//   • Raw pointers passed to it are valid, non-null, and properly aligned.
//   • The C code upholds Rust's aliasing rules (no data races, no use-after-
//     free, etc.).
//
// The safe wrappers in lib.rs are responsible for establishing all of these
// preconditions before crossing the boundary.
unsafe extern "C" {
    // `queens_count` takes and returns plain integers — no pointers, no global
    // state, well-defined behaviour for all inputs. In Rust 2024 we can mark
    // it `safe`, allowing callers to invoke it without an `unsafe` block.
    pub safe fn queens_count(n: c_int) -> c_int;

    // The remaining functions take raw pointers and therefore stay `unsafe`.
    // Callers must guarantee the pointed-to memory is valid and exclusively
    // accessible for the duration of the call.
    pub fn queens_find_first(n: c_int, board: *mut RawQueensBoard) -> c_int;
    pub fn queens_find_all(
        n: c_int,
        solutions: *mut RawQueensBoard,
        max_solutions: c_int,
    ) -> c_int;
    pub fn queens_is_valid(board: *const RawQueensBoard) -> c_int;
}
