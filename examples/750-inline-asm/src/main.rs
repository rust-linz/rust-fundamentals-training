//! Inline assembly with `asm!`
//!
//! Two tiny functions, each written twice: once for x86_64 (Intel/AMD PCs)
//! and once for aarch64 (ARM, e.g. Apple Silicon Macs). Assembly is tied to a
//! CPU architecture, so `#[cfg(target_arch = "...")]` picks the right version
//! at compile time.

use std::arch::asm;
use std::hint::black_box;

#[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
compile_error!("this example supports x86_64 and aarch64 only");

// ── 1) Add two numbers ───────────────────────────────────────────────────────
//
// The basics: `{a}` / `{b}` are placeholders that the compiler replaces with
// registers it picks itself (`reg`). Operands say how data flows:
//   in(reg) x            value goes into the asm
//   out(reg) x           value comes out of the asm
//   inout(reg) x => y    goes in, and the (changed) register comes out as y
//
// `options(...)` are promises to the optimizer. Breaking one is undefined
// behaviour, just like any other broken `unsafe` contract:
//   pure     no side effects; the result depends only on the inputs
//   nomem    does not read or write memory
//   nostack  does not push anything onto the stack

/// x86_64: `add dst, src` means `dst = dst + src` (two operands).
#[cfg(target_arch = "x86_64")]
fn add(a: u64, b: u64) -> u64 {
    let result: u64;
    // SAFETY: the instruction only uses the registers declared below.
    // It touches no memory and does not use the stack.
    unsafe {
        asm!(
            "add {a}, {b}",
            a = inout(reg) a => result,
            b = in(reg) b,
            options(pure, nomem, nostack),
        );
    }
    result
}

/// aarch64: `add dst, src1, src2` means `dst = src1 + src2` (three operands).
#[cfg(target_arch = "aarch64")]
fn add(a: u64, b: u64) -> u64 {
    let result: u64;
    // SAFETY: the instruction only uses the registers declared below.
    // It touches no memory and does not use the stack.
    unsafe {
        asm!(
            "add {r}, {a}, {b}",
            r = out(reg) result,
            a = in(reg) a,
            b = in(reg) b,
            options(pure, nomem, nostack),
        );
    }
    result
}

// ── 2) Read the CPU's tick counter ───────────────────────────────────────────
//
// Something plain Rust has no syntax for. Note that this asm is *not* `pure`:
// it returns a different value every time, so the compiler must not merge or
// remove calls.
//
// Tip: before writing asm, check `std::arch` for an intrinsic. On x86_64,
// `std::arch::x86_64::_rdtsc()` does exactly what the function below does.

/// x86_64: `rdtsc` always writes the result into the fixed registers
/// `edx` (high 32 bits) and `eax` (low 32 bits). Instead of `reg`, we name
/// those registers explicitly.
#[cfg(target_arch = "x86_64")]
fn cpu_ticks() -> u64 {
    let low: u32;
    let high: u32;
    // SAFETY: rdtsc only writes eax and edx, which are declared as outputs.
    // It touches no memory and does not use the stack.
    unsafe {
        asm!("rdtsc", out("eax") low, out("edx") high, options(nomem, nostack));
    }
    (u64::from(high) << 32) | u64::from(low)
}

/// aarch64: `mrs` reads the system register `cntvct_el0` (virtual counter)
/// into any general-purpose register.
#[cfg(target_arch = "aarch64")]
fn cpu_ticks() -> u64 {
    let ticks: u64;
    // SAFETY: mrs only writes the output register declared below. Linux and
    // macOS allow user-space programs to read cntvct_el0.
    unsafe {
        asm!("mrs {t}, cntvct_el0", t = out(reg) ticks, options(nomem, nostack));
    }
    ticks
}

fn main() {
    println!("Running on {}", std::env::consts::ARCH);

    println!("40 + 2 = {}", add(40, 2));

    // Time a small workload. The counter runs at a fixed, CPU-specific rate,
    // so the result is in ticks, not in seconds.
    let start = cpu_ticks();
    let mut sum = 0;
    for i in 0..1_000_000 {
        // black_box stops the compiler from computing the sum at compile time.
        sum = add(sum, black_box(i));
    }
    let end = cpu_ticks();
    println!("sum of 0..1_000_000 = {sum}, took {} ticks", end - start);
}
