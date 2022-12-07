# Progress Iterator example

Based on [Type-Driven API Design in Rust](https://www.youtube.com/watch?v=bnnacleqg6k) by Will Chrichton

This example implements:
- Progress struct that wraps the iterator
- `Iterator` for Progress to print the progress and forward the wrapped iterator's elements --> **Iterator**
- `ProgressIteratorExt`, so `Progress` is attached to each iterator who calles `.progress()` --> **Extension Traits**
- `Unbounded` and `Bounded` state structs
- `ProgressDisplay` to decide how `Bounded` and `Unbounded` should be displayed --> **Typestate**
- `Progress` starts `Unbounded`
- `.with_bounds` for all `Unbounded` to transform state into `Bounded`
- `.with_delims` for all `Bounded` to change delimiters
