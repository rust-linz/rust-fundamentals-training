/**
 * C library for solving the N-Queens problem.
 *
 * Placing n queens on an n×n board so that no queen can attack any other.
 *
 * Design notes for the FFI boundary:
 * - No dynamic memory allocation: all memory is owned and passed in by the
 *   caller, making ownership transparent across the language boundary.
 * - queens_board_t uses a fixed-size array (int positions[QUEENS_MAX_N])
 *   rather than a pointer so the struct is self-contained and requires no
 *   separate free().
 * - All functions return int (0/1 flag or count) for straightforward error
 *   checking from Rust.
 */

#ifndef QUEENS_H
#define QUEENS_H

/** Maximum board size supported by this library. */
#define QUEENS_MAX_N 16

/**
 * Represents one solution to the N-Queens problem.
 *
 * positions[row] is the 0-based column of the queen in that row.
 * Only positions[0..n-1] are meaningful; elements beyond n are uninitialized.
 *
 * The fixed-size array is intentional: it avoids heap allocation and makes
 * the struct safe to pass by value or pointer across the FFI boundary without
 * any custom memory-management protocol.
 */
typedef struct {
    int n;
    int positions[QUEENS_MAX_N];
} queens_board_t;

/**
 * Returns the total number of distinct solutions for the n-queens problem.
 * Returns 0 for n <= 0 or n > QUEENS_MAX_N.
 *
 * Known values: n=1→1, n=2→0, n=3→0, n=4→2, n=8→92.
 */
int queens_count(int n);

/**
 * Finds the first solution for the n-queens problem.
 * Writes the solution to *board.
 * Returns 1 if a solution was found, 0 if none exists or on invalid input.
 */
int queens_find_first(int n, queens_board_t *board);

/**
 * Finds up to max_solutions solutions for the n-queens problem.
 * The caller provides the output array; the function fills it.
 * Returns the number of solutions actually written (may be < max_solutions).
 */
int queens_find_all(int n, queens_board_t *solutions, int max_solutions);

/**
 * Returns 1 if board represents a valid n-queens solution, 0 otherwise.
 * Checks that no two queens share a row, column, or diagonal.
 */
int queens_is_valid(const queens_board_t *board);

#endif /* QUEENS_H */
