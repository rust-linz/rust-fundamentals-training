# 750 — Inline Assembly with `asm!`

Rust lets you write CPU instructions directly inside a function with the
[`asm!`](https://doc.rust-lang.org/reference/inline-assembly.html) macro
(stable since Rust 1.59). This example is intentionally tiny: two functions,
a few lines each, just to show the capability.

| Function | What it shows |
|---|---|
| `add(a, b)` | The basic syntax: placeholders, `in` / `out` / `inout` operands, `options(...)` |
| `cpu_ticks()` | Why you'd use asm at all: an instruction plain Rust has no syntax for, plus explicitly named registers |

## Run it

```bash
just run        # cargo run --release
just show-asm   # show where our instructions end up in the compiled code
just lint       # clippy -D warnings
```

Example output on x86_64:

```text
Running on x86_64
40 + 2 = 42
sum of 0..1_000_000 = 499999500000, took 3649900 ticks
```

## Anatomy of an `asm!` block

```rust
let result: u64;
// SAFETY: ...
unsafe {
    asm!(
        "add {a}, {b}",                 // the instruction(s), as a string
        a = inout(reg) a => result,     // operand: goes in and comes out
        b = in(reg) b,                  // operand: goes in only
        options(pure, nomem, nostack),  // promises to the optimizer
    );
}
```

- **`unsafe`**: the compiler can't check what the instructions do, so you
  have to. As in the C interop examples (700–702), every `unsafe` block has a
  `// SAFETY:` comment explaining why it is sound.
- **Placeholders** like `{a}` are replaced with real register names. With
  `reg`, the compiler picks a free register. Write a name such as `"eax"`
  instead when the instruction needs a specific register (see `rdtsc` in
  `cpu_ticks`).
- **Operands** describe the data flow: `in`, `out`, `inout`.
- **`options(...)`** are promises. `pure` + `nomem` let the optimizer treat
  the block like a normal calculation (move it, merge it, delete it when the
  result is unused). `cpu_ticks` is **not** `pure`, because it returns a
  different value on every call. A false promise is undefined behaviour.

## Assembly is not portable

Each function exists twice, selected with `#[cfg(target_arch = "...")]`:

| | x86_64 (Intel/AMD) | aarch64 (ARM, Apple Silicon) |
|---|---|---|
| Add | `add a, b` (2 operands: `a += b`) | `add r, a, b` (3 operands: `r = a + b`) |
| Tick counter | `rdtsc` → fixed registers `edx:eax` | `mrs x, cntvct_el0` → any register |

On any other architecture, `compile_error!` stops the build with a clear
message. Tick counts are not comparable between machines: each CPU's counter
runs at its own fixed rate.

## Seeing your instructions in the compiler output

`just show-asm` runs `cargo rustc --release -- --emit asm` and prints the parts
of the `.s` file between `#APP` and `#NO_APP`. Everything between those markers
comes from an `asm!` block. Things to notice:

- The compiler filled in the placeholders, e.g. `add rax, rcx`.
- The functions were inlined into `main`, but your instructions stay exactly as
  written.
- Some `#APP` / `#NO_APP` pairs are empty. They come from `black_box`, which is
  itself implemented as an empty `asm!` block that the optimizer can't see
  through.

On x86_64, the recipe asks for Intel syntax, which is also what `asm!` uses.
The default AT&T syntax would show `addq %rcx, %rax`, with the operands in
reverse order.

## When to use it (and when not)

Reach for `asm!` last. In this order, prefer:

1. Plain Rust. The optimizer usually beats hand-written assembly.
2. Intrinsics in [`std::arch`](https://doc.rust-lang.org/std/arch/index.html),
   e.g. `std::arch::x86_64::_rdtsc()` or SIMD functions. They are typed and
   the compiler understands them.
3. `asm!` for the rest: special CPU instructions, system registers, OS kernels,
   embedded and bare-metal code.

Related:
- `global_asm!` adds whole assembly functions at module level.
- `#[unsafe(naked)]` functions (stable since Rust 1.88) contain nothing but
  assembly, with no compiler-generated prologue or epilogue.
