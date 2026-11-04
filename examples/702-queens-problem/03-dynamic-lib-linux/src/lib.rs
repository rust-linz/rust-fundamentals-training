//! Safe Rust wrappers around the C queens solver.
//!
//! # Why safe wrappers?
//!
//! Functions declared in an `extern "C"` block are inherently `unsafe` —
//! Rust cannot check pointer validity, memory ownership, or C-side invariants.
//! The wrappers in this file form the "safe abstraction layer":
//!
//! - They validate inputs before calling C (e.g. `n == 0`, out-of-range `n`).
//! - They translate C conventions (raw pointers, int return codes) into
//!   idiomatic Rust types (`Option`, `Vec`, `bool`).
//! - After these checks, the `unsafe { }` block contains *only* the FFI call;
//!   any safety invariants required by C are documented with `// SAFETY:`.
//!
//! Callers of this module never need to write `unsafe`.

mod ffi;

use ffi::{RawQueensBoard, QUEENS_MAX_N};
use std::ffi::c_int;

// ── Public types ─────────────────────────────────────────────────────────────

/// One solution to the N-Queens problem.
///
/// `positions[row]` is the 0-based column of the queen in that row.
/// The board size is `positions.len()` — there is no separate `n` field
/// because it would always equal `positions.len()`, and a redundant field
/// could lead to inconsistencies.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueensBoard {
    /// `positions[row]` = column of the queen in `row` (0-based).
    /// The length of this vector is the board size.
    pub positions: Vec<usize>,
}

impl QueensBoard {
    /// Board size (number of queens / rows / columns).
    pub fn n(&self) -> usize {
        self.positions.len()
    }

    fn from_raw(raw: &RawQueensBoard) -> Self {
        // Clamp n to QUEENS_MAX_N: even if the C library has a bug and writes
        // an out-of-range n, this prevents a panic on the slice `[..n]`.
        // A safe wrapper must never crash because of a bad C value.
        let n = usize::try_from(raw.n).unwrap_or(0).min(QUEENS_MAX_N);
        // `try_from` for each c_int column value: column indices are always
        // non-negative (0..n-1), so this never returns Err in practice.
        let positions = raw.positions[..n]
            .iter()
            .map(|&c| usize::try_from(c).unwrap_or(0))
            .collect();
        Self { positions }
    }
}

/// Renders the board as a `Q`/`.` grid, one row per line.
impl std::fmt::Display for QueensBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let n = self.n();
        for row in 0..n {
            for col in 0..n {
                let ch = if self.positions[row] == col { 'Q' } else { '.' };
                write!(f, "{ch} ")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

// ── Public API ────────────────────────────────────────────────────────────────

/// Returns the total number of distinct solutions for the n-queens problem.
///
/// Returns 0 for `n == 0` or `n > 16`.
pub fn count_solutions(n: usize) -> usize {
    if n == 0 || n > QUEENS_MAX_N {
        return 0;
    }
    // `queens_count` is declared `safe` in ffi.rs (no pointers, no UB for any
    // integer input), so no `unsafe` block is needed here.
    let result = ffi::queens_count(c_int::try_from(n).expect("n <= 16 fits in c_int"));
    usize::try_from(result).unwrap_or(0)
}

/// Returns the first solution found, or `None` if no solution exists.
pub fn find_first(n: usize) -> Option<QueensBoard> {
    if n == 0 || n > QUEENS_MAX_N {
        return None;
    }
    let mut raw = RawQueensBoard::default();
    // SAFETY:
    //   • n is in 1..=16; the cast to c_int is lossless.
    //   • `raw` is a fully initialised local variable; its address is stable
    //     for the duration of the call (no reallocation can occur).
    //   • The C function writes exactly one queens_board_t into *board.
    let found = unsafe {
        ffi::queens_find_first(c_int::try_from(n).expect("n <= 16 fits in c_int"), &mut raw)
    };
    if found == 1 {
        Some(QueensBoard::from_raw(&raw))
    } else {
        None
    }
}

/// Returns up to `max` solutions.
///
/// Pass `max = 0` to retrieve **all** solutions (calls `count_solutions`
/// internally to size the buffer).
///
/// Returns an empty `Vec` for `n == 0` or `n > 16`.
///
/// **Note:** The buffer is pre-allocated to hold `max` (or all) solutions.
/// For large `n` this can require substantial memory (e.g. n=16 has 14,772,512
/// solutions, each 68 bytes ≈ 1 GB). Prefer a reasonable `max` for large `n`.
pub fn find_all(n: usize, max: usize) -> Vec<QueensBoard> {
    if n == 0 || n > QUEENS_MAX_N {
        return Vec::new();
    }
    let limit = if max == 0 { count_solutions(n) } else { max };
    if limit == 0 {
        return Vec::new();
    }

    // Allocate a heap buffer for the C function to write into.
    //
    // Why a Vec here?
    //   • We need a contiguous array whose address we can pass to C.
    //   • `as_mut_ptr()` yields a raw pointer to the first element.
    //   • The Vec is not reallocated while C runs (no other Rust code
    //     executes concurrently), so the pointer stays valid throughout.
    let mut raw_solutions = vec![RawQueensBoard::default(); limit];

    // SAFETY:
    //   • n is in 1..=16.
    //   • `raw_solutions.as_mut_ptr()` points to `limit` initialised,
    //     contiguous RawQueensBoard values — exactly what C expects.
    //   • The C function writes at most `limit` entries (max_solutions
    //     parameter), so no out-of-bounds write can occur.
    let found = unsafe {
        ffi::queens_find_all(
            c_int::try_from(n).expect("n <= 16 fits in c_int"),
            raw_solutions.as_mut_ptr(),
            c_int::try_from(limit).expect("limit fits in c_int"),
        )
    };
    let found = usize::try_from(found).unwrap_or(0);
    raw_solutions[..found]
        .iter()
        .map(QueensBoard::from_raw)
        .collect()
}

/// Returns `true` if `board` is a valid n-queens solution.
pub fn is_valid(board: &QueensBoard) -> bool {
    let n = board.n();
    if n == 0 || n > QUEENS_MAX_N {
        return false;
    }
    let mut raw = RawQueensBoard {
        n: c_int::try_from(n).expect("board.n() <= 16 fits in c_int"),
        ..Default::default()
    };
    for (i, &col) in board.positions.iter().enumerate() {
        // `col` comes from user input — it could be any usize. If it exceeds
        // c_int range, we substitute -1, which the C validator rejects
        // (its `c < 0 || c >= n` check), correctly returning "invalid".
        raw.positions[i] = c_int::try_from(col).unwrap_or(-1);
    }
    // SAFETY: `raw` is fully initialised; the pointer is valid and exclusively
    //         owned for the duration of this call.
    unsafe { ffi::queens_is_valid(&raw) == 1 }
}

// ── Demo ──────────────────────────────────────────────────────────────────────

/// Runs a demonstration of all four API functions, printing results to stdout.
///
/// Placed in `lib.rs` so `main.rs` stays minimal (one line) and is nearly
/// identical across the three example projects. Only the crate-name prefix
/// in `main.rs` differs between approaches.
pub fn run_demo() {
    println!("=== N-Queens Solver (Rust + C FFI) ===");
    println!();

    println!("Solution counts:");
    for n in 1usize..=10 {
        let count = count_solutions(n);
        println!("  {n:2}-queens: {count}");
    }

    println!();
    println!("First solution for 8-queens:");
    match find_first(8) {
        Some(board) => print!("{board}"),
        None => println!("  (no solution)"),
    }

    println!();
    println!("First 3 solutions for 6-queens:");
    for (i, board) in find_all(6, 3).iter().enumerate() {
        println!("Solution {}:", i + 1);
        print!("{board}");
    }

    let good = QueensBoard {
        positions: vec![1, 3, 0, 2],
    };
    let bad = QueensBoard {
        positions: vec![0, 0, 2, 3],
    };
    println!();
    let good_valid = is_valid(&good);
    let bad_valid = is_valid(&bad);
    println!("is_valid([1,3,0,2]): {good_valid}");
    println!("is_valid([0,0,2,3]): {bad_valid}");
}

// ── Tests ─────────────────────────────────────────────────────────────────────
//
// Unit tests live in the same file as the code they test. `#[cfg(test)]`
// ensures the test module is compiled only during `cargo test`, keeping the
// production binary lean.
//
// Because the tests call the public safe wrappers (not the raw FFI), they
// also exercise the C code underneath — every test is effectively an
// integration test of the Rust↔C boundary.

#[cfg(test)]
mod tests {
    use super::*;

    // ── count_solutions ───────────────────────────────────────────────────────

    #[test]
    fn count_n0_returns_0() {
        assert_eq!(count_solutions(0), 0);
    }

    #[test]
    fn count_n1_returns_1() {
        assert_eq!(count_solutions(1), 1);
    }

    #[test]
    fn count_n2_returns_0() {
        assert_eq!(count_solutions(2), 0);
    }

    #[test]
    fn count_n3_returns_0() {
        assert_eq!(count_solutions(3), 0);
    }

    #[test]
    fn count_n4_returns_2() {
        assert_eq!(count_solutions(4), 2);
    }

    #[test]
    fn count_n8_returns_92() {
        assert_eq!(count_solutions(8), 92);
    }

    // ── find_first ────────────────────────────────────────────────────────────

    #[test]
    fn find_first_n0_returns_none() {
        assert!(find_first(0).is_none());
    }

    #[test]
    fn find_first_n1_returns_single_queen() {
        let board = find_first(1).unwrap();
        assert_eq!(board.n(), 1);
        assert_eq!(board.positions, vec![0]);
    }

    #[test]
    fn find_first_n2_returns_none() {
        assert!(find_first(2).is_none());
    }

    #[test]
    fn find_first_n4_returns_valid_board() {
        let board = find_first(4).unwrap();
        assert_eq!(board.n(), 4);
        assert!(is_valid(&board));
    }

    #[test]
    fn find_first_n8_returns_valid_board() {
        let board = find_first(8).unwrap();
        assert_eq!(board.n(), 8);
        assert!(is_valid(&board));
    }

    // ── find_all ──────────────────────────────────────────────────────────────

    #[test]
    fn find_all_n0_returns_empty() {
        assert!(find_all(0, 10).is_empty());
    }

    #[test]
    fn find_all_n4_returns_all_2_solutions() {
        let solutions = find_all(4, 0);
        assert_eq!(solutions.len(), 2);
        for s in &solutions {
            assert!(is_valid(s));
        }
    }

    #[test]
    fn find_all_n8_returns_all_92_solutions() {
        let solutions = find_all(8, 0);
        assert_eq!(solutions.len(), 92);
        for s in &solutions {
            assert!(is_valid(s));
        }
    }

    #[test]
    fn find_all_n8_capped_at_10() {
        let solutions = find_all(8, 10);
        assert_eq!(solutions.len(), 10);
        for s in &solutions {
            assert!(is_valid(s));
        }
    }

    // ── is_valid ──────────────────────────────────────────────────────────────

    #[test]
    fn is_valid_known_good_4queens() {
        // [1,3,0,2]: row0→col1, row1→col3, row2→col0, row3→col2
        let board = QueensBoard {
            positions: vec![1, 3, 0, 2],
        };
        assert!(is_valid(&board));
    }

    #[test]
    fn is_valid_same_column_is_false() {
        // Queens at (row0,col0) and (row1,col0) share a column.
        let board = QueensBoard {
            positions: vec![0, 0, 2, 3],
        };
        assert!(!is_valid(&board));
    }

    #[test]
    fn is_valid_same_diagonal_is_false() {
        // Queens at (row0,col0) and (row1,col1) share a diagonal.
        let board = QueensBoard {
            positions: vec![0, 1, 3, 2],
        };
        assert!(!is_valid(&board));
    }

    #[test]
    fn is_valid_n1_single_queen() {
        let board = QueensBoard {
            positions: vec![0],
        };
        assert!(is_valid(&board));
    }

    #[test]
    fn is_valid_empty_board_returns_false() {
        let board = QueensBoard {
            positions: vec![],
        };
        assert!(!is_valid(&board));
    }
}
