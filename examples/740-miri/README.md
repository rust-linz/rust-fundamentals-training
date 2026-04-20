# Miri Highlights

[`Miri`](https://github.com/rust-lang/miri) is an interpreter for Rust programs that focuses on finding undefined behavior. It can run binaries and tests and report issues that the normal compiler and borrow checker cannot necessarily prove or catch. This is especially useful for code that uses `unsafe`, raw pointers, uninitialized memory, or low-level memory manipulation.

Miri is not a general proof of correctness. It tells you that a particular execution triggered undefined behavior, but a clean Miri run does not guarantee that the program is fully sound. Still, it is an excellent teaching and debugging tool because it makes many classes of memory and aliasing mistakes visible very quickly.

This sample is a small standalone Rust crate that demonstrates a few problems Miri is good at detecting. By default, the code is safe and all tests pass. The undefined-behavior variants are activated through Cargo features, so you can selectively switch on one bad example at a time and then run the tests under Miri.

## Setup

The sample uses a normal Cargo binary crate plus a `justfile` with helper commands. First install nightly Rust and the Miri component:

```sh
just install-miri
just miri-setup
```

After that, the safe default configuration can be checked with:

```sh
just test
just miri-test
just run
```

The crate defines three optional Cargo features:

- `ub-use-after-free`
- `ub-uninitialized-read`
- `ub-overlapping-copy`

Only one of these features should be enabled at a time. The code enforces that rule with a compile-time error.

## Samples

### Use-after-free

The first sample shows a classic dangling-pointer bug. In safe mode, the code keeps a `Box<i32>` alive and reads the value normally. When `ub-use-after-free` is enabled, the code converts the box into a raw pointer, frees the allocation, and then dereferences the dangling pointer.

This is exactly the kind of bug Miri is designed to catch. Running `just miri-use-after-free` triggers a Miri error that reports the pointer access after the allocation has already been freed.

### Uninitialized read

The second sample uses `MaybeUninit<i32>`. In safe mode, the value is created with `MaybeUninit::new(42)` and then read with `assume_init()`, which is valid because the memory really was initialized first. With `ub-uninitialized-read` enabled, the code creates uninitialized storage and immediately reads from it.

Normal compilation does not reject this pattern because `MaybeUninit` is specifically designed for low-level initialization workflows. Miri, however, detects that the program reads memory that was never initialized and stops with an error.

### Overlapping `copy_nonoverlapping`

The third sample demonstrates an intrinsic precondition violation. In safe mode, the code shifts bytes inside an array with `copy_within`, which is the correct API for overlapping ranges. With `ub-overlapping-copy` enabled, the code calls `std::ptr::copy_nonoverlapping` on source and destination regions that overlap.

The important point here is that the function name is literal: `copy_nonoverlapping` may only be used when the ranges do not overlap. Running `just miri-overlapping-copy` makes Miri report that this precondition was violated.
