#include "queens.h"
#include <limits.h>
#include <stddef.h>
#include <stdlib.h>
#include <string.h>

/* ── Internal helpers ────────────────────────────────────────────────────── */

/**
 * Returns 1 if placing a queen at (row, col) is safe given the queens already
 * placed in rows 0..row-1 at positions[0..row-1].
 */
static int is_safe(const int *positions, int row, int col) {
    for (int r = 0; r < row; r++) {
        int c = positions[r];
        if (c == col) return 0;                    /* same column */
        if (abs(c - col) == abs(r - row)) return 0; /* same diagonal */
    }
    return 1;
}

/**
 * Core recursive backtracking solver.
 *
 * n             - board size
 * row           - current row being filled (0-based)
 * positions     - partial solution; positions[0..row-1] are already set
 * solutions     - output array (NULL when only counting)
 * max_solutions - stop after writing this many solutions
 * count         - running count of solutions found so far (in/out)
 *
 * Returns 1 to signal early termination (max_solutions reached), 0 otherwise.
 */
static int solve(int n, int row, int *positions,
                 queens_board_t *solutions, int max_solutions, int *count) {
    if (row == n) {
        /* Complete solution found */
        if (solutions != NULL && *count < max_solutions) {
            solutions[*count].n = n;
            memcpy(solutions[*count].positions, positions,
                   (size_t)n * sizeof(int));
        }
        (*count)++;
        return (*count >= max_solutions); /* signal early exit when capped */
    }
    for (int col = 0; col < n; col++) {
        if (is_safe(positions, row, col)) {
            positions[row] = col;
            if (solve(n, row + 1, positions, solutions, max_solutions, count)) {
                return 1;
            }
        }
    }
    return 0;
}

/* ── Public API ──────────────────────────────────────────────────────────── */

int queens_count(int n) {
    if (n <= 0 || n > QUEENS_MAX_N) return 0;
    int positions[QUEENS_MAX_N] = {0};
    int count = 0;
    /* Pass INT_MAX so solve() never stops early; solutions=NULL means
     * we only count without storing. */
    solve(n, 0, positions, NULL, INT_MAX, &count);
    return count;
}

int queens_find_first(int n, queens_board_t *board) {
    if (n <= 0 || n > QUEENS_MAX_N || board == NULL) return 0;
    int positions[QUEENS_MAX_N] = {0};
    int count = 0;
    /* max_solutions=1 makes solve() stop after the first solution. */
    solve(n, 0, positions, board, 1, &count);
    return count > 0 ? 1 : 0;
}

int queens_find_all(int n, queens_board_t *solutions, int max_solutions) {
    if (n <= 0 || n > QUEENS_MAX_N || solutions == NULL || max_solutions <= 0)
        return 0;
    int positions[QUEENS_MAX_N] = {0};
    int count = 0;
    solve(n, 0, positions, solutions, max_solutions, &count);
    return count;
}

int queens_is_valid(const queens_board_t *board) {
    if (board == NULL) return 0;
    int n = board->n;
    if (n <= 0 || n > QUEENS_MAX_N) return 0;
    for (int r = 0; r < n; r++) {
        int c = board->positions[r];
        if (c < 0 || c >= n) return 0;
        for (int r2 = r + 1; r2 < n; r2++) {
            int c2 = board->positions[r2];
            if (c == c2) return 0;                        /* same column   */
            if (abs(c - c2) == abs(r - r2)) return 0;    /* same diagonal */
        }
    }
    return 1;
}
